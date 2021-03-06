use std::ffi::OsString;
use std::iter::once;

use super::ToolCommand;
use crate::error::ErrorDetails;
use crate::session::{ActivityKind, Session};

use volta_fail::{throw, Fallible};

pub(super) fn command<A>(exe: OsString, args: A, session: &mut Session) -> Fallible<ToolCommand>
where
    A: IntoIterator<Item = OsString>,
{
    session.add_event_start(ActivityKind::Binary);

    // first try to use the project toolchain
    if let Some(project) = session.project()? {
        // check if the executable is a direct dependency
        if project.has_direct_bin(&exe)? {
            // use the full path to the file
            let mut path_to_bin = project.local_bin_dir();
            path_to_bin.push(&exe);

            if !path_to_bin.is_file() {
                throw!(ErrorDetails::ProjectLocalBinaryNotFound {
                    command: path_to_bin.to_string_lossy().to_string(),
                });
            }

            let path_to_bin = path_to_bin.as_os_str();

            // if we're in a pinned project, use the project's platform.
            if let Some(ref platform) = project.platform() {
                let image = platform.checkout(session)?;
                let path = image.path()?;
                return Ok(ToolCommand::project_local(&path_to_bin, args, &path));
            }

            // otherwise use the user platform.
            if let Some(ref platform) = session.user_platform()? {
                let image = platform.checkout(session)?;
                let path = image.path()?;
                return Ok(ToolCommand::project_local(&path_to_bin, args, &path));
            }

            // if there's no user platform selected, pass through to existing PATH.
            return ToolCommand::passthrough(&path_to_bin, args, ErrorDetails::NoPlatform);
        }
    }

    // try to use the user toolchain
    if let Some(user_tool) = session.get_user_tool(&exe)? {
        let path = user_tool.image.path()?;
        let tool_path = user_tool.bin_path.into_os_string();
        let cmd = match user_tool.loader {
            Some(loader) => ToolCommand::direct(
                loader.command.as_ref(),
                loader
                    .args
                    .iter()
                    .map(|arg| OsString::from(arg))
                    .chain(once(tool_path))
                    .chain(args),
                &path,
            ),
            None => ToolCommand::direct(&tool_path, args, &path),
        };
        return Ok(cmd);
    }

    // at this point, there is no project or user toolchain
    // Pass through to the existing PATH
    ToolCommand::passthrough(
        &exe,
        args,
        ErrorDetails::BinaryNotFound {
            name: exe.to_string_lossy().to_string(),
        },
    )
}
