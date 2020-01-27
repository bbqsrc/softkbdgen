use crate::{models::DesktopModes, Load, ProjectBundle, xkb::*};
use log::{debug, log_enabled};
use snafu::{ResultExt, Snafu};
use snafu_cli_debug::SnafuCliDebug;
use std::{
    collections::BTreeMap,
    convert::TryFrom,
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

pub fn kbdgen_to_xkb(input: &Path, output: &Path, options: &Options) -> Result<(), Error> {
    let bundle = ProjectBundle::load(input).context(CannotLoad)?;
    if log_enabled!(log::Level::Debug) {
        debug!("Bundle `{}` loaded", input.display());
        let locales = bundle
            .project
            .locales
            .values()
            .map(|l| l.name.as_str())
            .collect::<Vec<_>>();
        debug!("Bundle contains these locales: {:?}", locales);
    }

    bundle
        .layouts
        .iter()
        .map(|(name, layout)| (name, layout_to_xkb_symbols(&name, layout, &bundle)))
        .try_for_each(|(name, symbols)| {
            let path = output.join(name).join("linux").with_extension("xkb");
            std::fs::create_dir_all(path.parent().unwrap())
                .context(CannotCreateFile { path: path.clone() })?;
            let file = File::create(&path).context(CannotCreateFile { path: path.clone() })?;
            debug!("Created file `{}`", path.display());
            let mut writer = BufWriter::new(file);
            symbols?
                .write_xkb(&mut writer)
                .context(CannotSerializeXkb)?;
            log::info!("Wrote to file `{}`", path.display());
            Ok(())
        })
        .context(CannotBeSaved)?;

    Ok(())
}

fn layout_to_xkb_symbols(
    name: &str,
    layout: &crate::models::Layout,
    project: &crate::ProjectBundle,
) -> Result<Symbols, SavingError> {
    Ok(Symbols {
        name: layout.display_names.get("en").cloned().unwrap_or_else(|| "lol".into()),
        groups: Vec::new(),
    })
}

#[derive(Debug, Clone)]
pub struct Options {
    pub standalone: bool,
}

#[derive(Snafu, SnafuCliDebug)]
pub enum Error {
    #[snafu(display("Could not load kbdgen bundle"))]
    CannotLoad {
        source: crate::LoadError,
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Could not write XKB file"))]
    CannotBeSaved {
        source: SavingError,
        backtrace: snafu::Backtrace,
    },
}

#[derive(Snafu, Debug)]
pub enum SavingError {
    #[snafu(display("Could not create file `{}`", path.display()))]
    CannotCreateFile {
        path: PathBuf,
        source: std::io::Error,
        backtrace: snafu::Backtrace,
    },
    #[snafu(display("Could transform to XKB"))]
    CannotSerializeXkb {
        source: std::io::Error,
        backtrace: snafu::Backtrace,
    },
}