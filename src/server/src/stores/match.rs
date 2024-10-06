use async_compression::tokio::write::GzipEncoder;
use tokio::fs::File;
use core::r#match::MatchResult;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

const CHUNK_SIZE: usize = 8 * 1024;
const MATCH_DIRECTORY: &str = "matches";

pub struct MatchStore;

impl MatchStore {
    pub async fn get(league_slug: &str, match_id: &str) -> Vec<u8> {
        let match_file = format!("{}/{}/{}.json.gz", MATCH_DIRECTORY, league_slug, match_id);

        let mut file = File::options()
            .read(true)
            .open(match_file)
            .await
            .unwrap();

        let mut result = Vec::new();

        file.read_to_end(&mut result).await.unwrap();

        result
    }

    pub async fn store(result: MatchResult) {
        let out_dir = format!("{}/{}", MATCH_DIRECTORY, result.league_slug);

        if let Ok(_) = tokio::fs::create_dir_all(&out_dir).await {
        }

        let out_file = format!("{}/{}.json.gz", out_dir, result.id);

        let file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(out_file)
            .await
            .unwrap();

        let mut compressed_file = GzipEncoder::new(file);

        if let Some(res) = result.details {
            //serialize and write compressed data
            let file_data = serde_json::to_vec(&res.position_data).unwrap();

            compressed_file.write_all(&file_data).await.unwrap();
            compressed_file.shutdown().await.unwrap();
        }
    }
}