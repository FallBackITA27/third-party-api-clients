use anyhow::Result;

use crate::Client;

pub struct Revisions {
    client: Client,
}

impl Revisions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Revisions { client }
    }

    /**
     * This function performs a `GET` to the `/files/{fileId}/revisions` endpoint.
     *
     * Lists a file's revisions.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `page_size: i64` -- The maximum number of revisions to return per page.
     * * `page_token: &str` -- The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
     */
    pub async fn drive_list(
        &self,
        file_id: &str,
        page_size: i64,
        page_token: &str,
    ) -> Result<Vec<crate::types::Revision>> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if page_size > 0 {
            query_args.push(format!("page_size={}", page_size));
        }
        if !page_token.is_empty() {
            query_args.push(format!("page_token={}", page_token));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/files/{}/revisions?{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            query_
        );

        let resp: crate::types::RevisionList = self.client.get(&url, None).await.unwrap();

        // Return our response data.
        Ok(resp.revisions)
    }

    /**
     * This function performs a `GET` to the `/files/{fileId}/revisions` endpoint.
     *
     * As opposed to `drive_list`, this function returns all the pages of the request at once.
     *
     * Lists a file's revisions.
     */
    pub async fn drive_list_revisions(&self, file_id: &str) -> Result<Vec<crate::types::Revision>> {
        let url = format!(
            "/files/{}/revisions",
            crate::progenitor_support::encode_path(&file_id.to_string()),
        );

        let mut resp: crate::types::RevisionList = self.client.get(&url, None).await.unwrap();

        let mut revisions = resp.revisions;
        let mut page = resp.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?pageToken={}", url, page), None)
                    .await
                    .unwrap();
            } else {
                resp = self
                    .client
                    .get(&format!("{}&pageToken={}", url, page), None)
                    .await
                    .unwrap();
            }

            revisions.append(&mut resp.revisions);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(revisions)
    }

    /**
     * This function performs a `GET` to the `/files/{fileId}/revisions/{revisionId}` endpoint.
     *
     * Gets a revision's metadata or content by ID.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `revision_id: &str` -- A link to this theme's background image.
     * * `acknowledge_abuse: bool` -- Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media.
     */
    pub async fn drive_get(
        &self,
        file_id: &str,
        revision_id: &str,
        acknowledge_abuse: bool,
    ) -> Result<crate::types::Revision> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if acknowledge_abuse {
            query_args.push(format!("acknowledge_abuse={}", acknowledge_abuse));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/files/{}/revisions/{}?{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            crate::progenitor_support::encode_path(&revision_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `DELETE` to the `/files/{fileId}/revisions/{revisionId}` endpoint.
     *
     * Permanently deletes a file version. You can only delete revisions for files with binary content in Google Drive, like images or videos. Revisions for other files, like Google Docs or Sheets, and the last remaining file version can't be deleted.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `revision_id: &str` -- A link to this theme's background image.
     */
    pub async fn drive_delete(&self, file_id: &str, revision_id: &str) -> Result<()> {
        let url = format!(
            "/files/{}/revisions/{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            crate::progenitor_support::encode_path(&revision_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `PATCH` to the `/files/{fileId}/revisions/{revisionId}` endpoint.
     *
     * Updates a revision with patch semantics.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `revision_id: &str` -- A link to this theme's background image.
     */
    pub async fn drive_update(
        &self,
        file_id: &str,
        revision_id: &str,
        body: &crate::types::Revision,
    ) -> Result<crate::types::Revision> {
        let url = format!(
            "/files/{}/revisions/{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            crate::progenitor_support::encode_path(&revision_id.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}