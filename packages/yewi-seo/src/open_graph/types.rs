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

impl MusicType {
  pub fn to_string(self) -> String {
    match self {
      MusicType::Song => "music.song".to_string(),
      MusicType::Album => "music.album".to_string(),
      MusicType::Playlist => "music.playlist".to_string(),
      MusicType::RadioStation => "music.radio_station".to_string(),
    }
  }
}

impl VideoType {
  pub fn to_string(self) -> String {
    match self {
      VideoType::TvShow => "video.tv_show".to_string(),
      VideoType::Other => "video.other".to_string(),
      VideoType::Movie => "video.movie".to_string(),
      VideoType::Episode => "video.episode".to_string(),
    }
  }
}

impl OpenGraphDeterminer {
  pub fn to_string(self) -> String {
    match self {
      OpenGraphDeterminer::An => "an".to_string(),
      OpenGraphDeterminer::A => "a".to_string(),
      OpenGraphDeterminer::The => "the".to_string(),
      OpenGraphDeterminer::Auto => "auto".to_string(),
      OpenGraphDeterminer::None => "".to_string(),
    }
  }
}

impl OpenGraphType {
  pub fn to_string(self) -> String {
    match self {
      OpenGraphType::Article => "article".to_string(),
      OpenGraphType::Book => "book".to_string(),
      OpenGraphType::Profile => "profile".to_string(),
      OpenGraphType::Website => "website".to_string(),
      OpenGraphType::Video(video_type) => video_type.to_string(),
      OpenGraphType::Music(music_type) => music_type.to_string(),
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