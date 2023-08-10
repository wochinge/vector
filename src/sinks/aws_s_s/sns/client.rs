use super::{Client, SendMessageEntry, SendMessageResponse};
use aws_sdk_sns::{error::PublishError, types::SdkError};
use futures::TryFutureExt;
use tracing::Instrument;

#[derive(Clone, Debug)]
pub struct SnsMessagePublisher {
    client: aws_sdk_sns::Client,
    topic_arn: String,
}

impl SnsMessagePublisher {
    pub fn new(client: aws_sdk_sns::Client, topic_arn: String) -> Self {
        Self { client, topic_arn }
    }
}
#[async_trait::async_trait]
impl Client<PublishError> for SnsMessagePublisher {
    async fn send_message(
        &self,
        entry: SendMessageEntry,
        byte_size: usize,
    ) -> Result<SendMessageResponse, SdkError<PublishError>> {
        self.client
            .publish()
            .message(entry.message_body)
            .set_message_group_id(entry.message_group_id)
            .set_message_deduplication_id(entry.message_deduplication_id)
            .topic_arn(self.topic_arn.clone())
            .send()
            .map_ok(|_| SendMessageResponse {
                byte_size,
                json_byte_size: entry
                    .metadata
                    .events_estimated_json_encoded_byte_size()
                    .clone(),
            })
            .instrument(info_span!("request").or_current())
            .await
    }
}
