use std::fmt::{Display, Formatter};
use crate::traits::ObjToIter;

#[derive(Debug, Clone)]
pub enum MusicType {
  Song,
  Album,
  Playlist,
  RadioStation,
}

#[derive(Debug, Clone)]
pub enum VideoType {
  TvShow,
  Other,
  Movie,
  Episode,
}
#[derive(Debug, Clone)]
pub enum OpenGraphType {
  Article,
  Book,
  Profile,
  Website,
  Video(VideoType),
  Music(MusicType),
}

#[derive(Debug, Clone)]
pub enum OpenGraphDeterminer {
  An,
  A,
  The,
  Auto,
  None,
}

impl Display for MusicType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      MusicType::Song => write!(f, "music.song"),
      MusicType::Album => write!(f, "music.album"),
      MusicType::Playlist => write!(f, "music.playlist"),
      MusicType::RadioStation => write!(f, "music.radio_station"),
    }
  }
}

impl Display for VideoType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      VideoType::TvShow => write!(f, "video.tv_show"),
      VideoType::Other => write!(f, "video.other"),
      VideoType::Movie => write!(f, "video.movie"),
      VideoType::Episode => write!(f, "video.episode")
    }
  }
}

impl Display for  OpenGraphDeterminer {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      OpenGraphDeterminer::An => write!(f, "an"),
      OpenGraphDeterminer::A => write!(f, "a"),
      OpenGraphDeterminer::The => write!(f, "the"),
      OpenGraphDeterminer::Auto => write!(f, "auto"),
      OpenGraphDeterminer::None => write!(f, ""),
    }
  }
}

impl Display for OpenGraphType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      OpenGraphType::Article => write!(f, "article"),
      OpenGraphType::Book => write!(f, "book"),
      OpenGraphType::Profile => write!(f, "profile"),
      OpenGraphType::Website => write!(f, "website"),
      OpenGraphType::Video(video_type) => write!(f, "{}", video_type),
      OpenGraphType::Music(music_type) => write!(f, "{}", music_type),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct OpenGraphProps {
  pub title: Option<String>,
  pub description: Option<String>,
  pub image: Option<String>,
  pub image_width: Option<String>,
  pub image_height: Option<String>,
  pub image_alt: Option<String>,
  pub image_type: Option<String>,
  pub image_secure_url: Option<String>,
  pub url: Option<String>,
  pub type_: Option<OpenGraphType>,
  pub site_name: Option<String>,
  pub locale: Option<String>,
  pub alternate_locale: Option<Vec<String>>,
  pub audio: Option<String>,
  pub audio_secure_url: Option<String>,
  pub audio_type: Option<String>,
  pub video: Option<String>,
  pub video_width: Option<String>,
  pub video_height: Option<String>,
  pub video_type: Option<String>,
  pub video_secure_url: Option<String>,
  pub country_name: Option<String>,
  pub determiner: Option<OpenGraphDeterminer>,
}

impl ObjToIter for OpenGraphProps {
  fn to_iter(self) -> Vec<(String, Option<String>)> {
    let alternate_locales =       self.alternate_locale.map(|locales| {
      locales
        .into_iter()
        .map(|locale| ("locale:alternate".to_string(), Some(locale)))
        .collect::<Vec<(String, Option<String>)>>()
    });
    let mut og_vec = vec![
      ("title".to_string(), self.title),
      ("description".to_string(), self.description),
      ("image".to_string(), self.image),
      ("image:width".to_string(), self.image_width),
      ("image:height".to_string(), self.image_height),
      ("image:alt".to_string(), self.image_alt),
      ("image:type".to_string(), self.image_type),
      ("image:secure_url".to_string(), self.image_secure_url),
      ("url".to_string(), self.url),
      ("type_".to_string(), self.type_.map(|t| t.to_string())),
      ("site_name".to_string(), self.site_name),
      ("locale".to_string(), self.locale),
      ("audio".to_string(), self.audio),
      ("audio:secure_url".to_string(), self.audio_secure_url),
      ("audio:type".to_string(), self.audio_type),
      ("video".to_string(), self.video),
      ("video:width".to_string(), self.video_width.map(|w| w.to_string())),
      ("video:height".to_string(), self.video_height.map(|h| h.to_string())),
      ("video:type".to_string(), self.video_type),
      ("video:secure_url".to_string(), self.video_secure_url),
      ("country_name".to_string(), self.country_name),
      ("determiner".to_string(), self.determiner.map(|d| d.to_string())),
    ];
    if let Some(alternate_locales) = alternate_locales {
      og_vec.extend(alternate_locales);
    }

    og_vec
  }
}