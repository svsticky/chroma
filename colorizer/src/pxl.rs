use color_eyre::eyre::Error;
use color_eyre::Result;
use regex::Regex;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use tracing::trace;

pub struct PxlFileTree {
    base: PathBuf,
}

pub struct PxlAlbum {
    dir: PathBuf,
    pub name: String,
}

pub struct PxlPhoto {
    pub s3_url: String,
}

impl PxlFileTree {
    pub fn new(base: PathBuf) -> Self {
        Self { base }
    }

    pub fn get_albums(&self) -> Result<Vec<PxlAlbum>> {
        let rd = fs::read_dir(&self.base)?;
        let album_title_regex = Regex::new("<title>(.*)</title>")?;

        let albums = rd
            .into_iter()
            .map(|dir| {
                let dir = dir?;
                let path = dir.path();

                let album_index = path.join("index.html");
                trace!("Parsing album page {album_index:?} for album title");

                let title = Self::parse_album_title(&album_index, &album_title_regex)?;

                Ok(PxlAlbum {
                    dir: path,
                    name: title,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(albums)
    }

    fn parse_album_title(path: &Path, regex: &Regex) -> Result<String> {
        let mut f = fs::File::open(path)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        let capts = regex
            .captures(&contents)
            .ok_or(Error::msg("Album page has no title"))?;

        let title = capts
            .get(1)
            .ok_or(Error::msg("Album page has no title"))?
            .as_str()
            .to_string();

        Ok(title)
    }
}

impl PxlAlbum {
    pub fn get_photos(&self) -> Result<Vec<PxlPhoto>> {
        let rd = fs::read_dir(&self.dir)?;
        let photo_src_regex = Regex::new(r#"<img src="(.*)""#)?;

        let photos = rd
            .into_iter()
            .map(|dir| Ok(dir?.path()))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .filter(|path| path.is_dir())
            .map(|album_dir| {
                let photo_index = album_dir.join("index.html");
                trace!("Parsing photo file {photo_index:?} for image src");

                let src = Self::parse_image_src(&photo_index, &photo_src_regex)?;
                Ok(PxlPhoto { s3_url: src })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(photos)
    }

    fn parse_image_src(file: &Path, regex: &Regex) -> Result<String> {
        let mut f = fs::File::open(file)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        let capts = regex
            .captures(&contents)
            .ok_or(Error::msg("Image has no image source"))?;

        let title = capts
            .get(1)
            .ok_or(Error::msg("Image has no image source"))?
            .as_str()
            .to_string();

        Ok(title)
    }
}
