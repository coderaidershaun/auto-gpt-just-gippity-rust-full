import requests
from decouple import config
import json

def fetch_youtube_videos():
    api_key = config("GOOGLE_SEARCH_API_KEY")
    url = "https://www.googleapis.com/youtube/v3/search"
    params = {
        "part": "snippet",
        "maxResults": 10,
        "type": "video",
        "key": api_key,
        "q": "programming"
    }
    response = requests.get(url, params=params)
    videos = response.json()

    with open("output.json", "w") as f:
        json.dump(videos, f)

fetch_youtube_videos()