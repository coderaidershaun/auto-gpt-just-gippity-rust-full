use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnail {
    pub height: u32,
    pub url: String,
    pub width: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnails {
    pub default: Thumbnail,
    pub high: Thumbnail,
    pub medium: Thumbnail,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Snippet {
    pub channelId: String,
    pub channelTitle: String,
    pub description: String,
    pub liveBroadcastContent: String,
    pub publishTime: String,
    pub publishedAt: String,
    pub thumbnails: Thumbnails,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoId {
    pub kind: String,
    pub videoId: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub etag: String,
    pub id: VideoId,
    pub kind: String,
    pub snippet: Snippet,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageInfo {
    pub resultsPerPage: u32,
    pub totalResults: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct YTSearchResponse {
    pub etag: String,
    pub items: Vec<Item>,
    pub kind: String,
    pub nextPageToken: String,
    pub pageInfo: PageInfo,
    pub regionCode: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct APIEndpointKey {
    pub endpoint: String,
    pub api_key_available: u8
}
