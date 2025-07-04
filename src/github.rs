use crate::core::config::GitHubConfig;
use crate::core::error::SyncError;
use crate::core::task::Task;
use crate::core::{EddaError, EddaResult};
use crate::sync::{SyncProvider, SyncStatus};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// GitHub API client for integration
pub struct GitHubClient {
    client: Client,
    config: GitHubConfig,
    base_url: String,
    owner: String,
    repo: String,
}

/// GitHub issue representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubIssue {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub body: Option<String>,
    pub state: String,
    pub labels: Vec<GitHubLabel>,
    pub assignees: Vec<GitHubUser>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub milestone: Option<GitHubMilestone>,
    pub comments: u64,
    pub html_url: String,
}

/// GitHub label representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubLabel {
    pub id: u64,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
}

/// GitHub user representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubUser {
    pub id: u64,
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
}

/// GitHub milestone representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubMilestone {
    pub id: u64,
    pub number: u64,
    pub title: String,
    pub description: Option<String>,
    pub state: String,
    pub due_on: Option<DateTime<Utc>>,
}

/// GitHub project board representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubProject {
    pub id: u64,
    pub number: u64,
    pub name: String,
    pub body: Option<String>,
    pub state: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub columns: Vec<GitHubProjectColumn>,
}

/// GitHub project column representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubProjectColumn {
    pub id: u64,
    pub name: String,
    pub cards: Vec<GitHubProjectCard>,
}

/// GitHub project card representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubProjectCard {
    pub id: u64,
    pub note: Option<String>,
    pub content_url: Option<String>,
    pub column_id: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// GitHub workflow representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubWorkflow {
    pub id: u64,
    pub name: String,
    pub path: String,
    pub state: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// GitHub workflow run representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubWorkflowRun {
    pub id: u64,
    pub name: String,
    pub status: String,
    pub conclusion: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub head_branch: String,
    pub head_sha: String,
}

impl GitHubClient {
    /// Create a new GitHub client
    pub fn new(config: GitHubConfig) -> Result<Self, EddaError> {
        // Get token from environment variables
        let token = crate::core::config::get_github_token()
            .ok_or_else(|| EddaError::Sync(SyncError::Authentication {
                message: "GitHub token not found. Set GITHUB_TOKEN, EDDA_GITHUB_TOKEN, GH_TOKEN, or GITHUB_ACCESS_TOKEN environment variable.".to_string(),
            }))?;

        let repository = config.repository.clone().ok_or_else(|| {
            EddaError::Sync(SyncError::Configuration {
                message: "GitHub repository not configured".to_string(),
            })
        })?;

        let (owner, repo) = repository.split_once('/').ok_or_else(|| {
            EddaError::Sync(SyncError::Configuration {
                message: format!(
                    "Invalid repository format: {}. Expected 'owner/repo'",
                    repository
                ),
            })
        })?;

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Authorization",
            format!("Bearer {}", token).parse().unwrap(),
        );
        headers.insert("User-Agent", "Edda/1.0".parse().unwrap());
        headers.insert("Accept", "application/vnd.github.v3+json".parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| {
                EddaError::Sync(SyncError::Network {
                    message: format!("Failed to create HTTP client: {}", e),
                })
            })?;

        Ok(Self {
            client,
            config,
            base_url: "https://api.github.com".to_string(),
            owner: owner.to_string(),
            repo: repo.to_string(),
        })
    }

    /// Get issues from a repository
    pub async fn get_issues(&self, state: Option<&str>) -> EddaResult<Vec<GitHubIssue>> {
        let state = state.unwrap_or("open");
        let url = format!(
            "{}/repos/{}/{}/issues?state={}",
            self.base_url, self.owner, self.repo, state
        );

        let response = self.client.get(&url).send().await.map_err(|e| {
            EddaError::Sync(SyncError::Network {
                message: format!("Failed to fetch GitHub issues: {}", e),
            })
        })?;

        if !response.status().is_success() {
            return Err(EddaError::Sync(SyncError::Network {
                message: format!(
                    "GitHub API error: {} {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            }));
        }

        let issues: Vec<GitHubIssue> = response.json().await.map_err(|e| {
            EddaError::Sync(SyncError::Network {
                message: format!("Failed to parse GitHub issues: {}", e),
            })
        })?;

        Ok(issues)
    }

    /// Create a new issue
    pub async fn create_issue(&self, title: &str, body: Option<&str>) -> EddaResult<GitHubIssue> {
        let url = format!(
            "{}/repos/{}/{}/issues",
            self.base_url, self.owner, self.repo
        );

        let payload = serde_json::json!({
            "title": title,
            "body": body,
        });

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                EddaError::Sync(SyncError::Network {
                    message: format!("Failed to create GitHub issue: {}", e),
                })
            })?;

        if !response.status().is_success() {
            return Err(EddaError::Sync(SyncError::Network {
                message: format!(
                    "GitHub API error: {} {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            }));
        }

        let issue: GitHubIssue = response.json().await.map_err(|e| {
            EddaError::Sync(SyncError::Network {
                message: format!("Failed to parse GitHub issue: {}", e),
            })
        })?;

        Ok(issue)
    }

    /// Update an existing issue
    pub async fn update_issue(
        &self,
        issue_number: u64,
        title: Option<&str>,
        body: Option<&str>,
        state: Option<&str>,
    ) -> EddaResult<GitHubIssue> {
        let url = format!(
            "{}/repos/{}/{}/issues/{}",
            self.base_url, self.owner, self.repo, issue_number
        );

        let mut payload = serde_json::Map::new();
        if let Some(title) = title {
            payload.insert(
                "title".to_string(),
                serde_json::Value::String(title.to_string()),
            );
        }
        if let Some(body) = body {
            payload.insert(
                "body".to_string(),
                serde_json::Value::String(body.to_string()),
            );
        }
        if let Some(state) = state {
            payload.insert(
                "state".to_string(),
                serde_json::Value::String(state.to_string()),
            );
        }

        let response = self
            .client
            .patch(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                EddaError::Sync(SyncError::Network {
                    message: format!("Failed to update GitHub issue: {}", e),
                })
            })?;

        if !response.status().is_success() {
            return Err(EddaError::Sync(SyncError::Network {
                message: format!(
                    "GitHub API error: {} {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            }));
        }

        let issue: GitHubIssue = response.json().await.map_err(|e| {
            EddaError::Sync(SyncError::Network {
                message: format!("Failed to parse GitHub issue: {}", e),
            })
        })?;

        Ok(issue)
    }

    /// Close an issue
    pub async fn close_issue(&self, issue_number: u64) -> EddaResult<GitHubIssue> {
        self.update_issue(issue_number, None, None, Some("closed"))
            .await
    }

    /// Get project boards
    pub async fn get_projects(&self) -> EddaResult<Vec<GitHubProject>> {
        let url = format!(
            "{}/repos/{}/{}/projects",
            self.base_url, self.owner, self.repo
        );

        let response = self.client.get(&url).send().await.map_err(|e| {
            EddaError::Sync(SyncError::Network {
                message: format!("Failed to fetch GitHub projects: {}", e),
            })
        })?;

        if !response.status().is_success() {
            return Err(EddaError::Sync(SyncError::Network {
                message: format!(
                    "GitHub API error: {} {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            }));
        }

        let projects: Vec<GitHubProject> = response.json().await.map_err(|e| {
            EddaError::Sync(SyncError::Network {
                message: format!("Failed to parse GitHub projects: {}", e),
            })
        })?;

        Ok(projects)
    }

    /// Get project columns
    pub async fn get_project_columns(
        &self,
        project_id: u64,
    ) -> EddaResult<Vec<GitHubProjectColumn>> {
        let url = format!("{}/projects/{}/columns", self.base_url, project_id);

        let response = self.client.get(&url).send().await.map_err(|e| {
            EddaError::Sync(SyncError::Network {
                message: format!("Failed to fetch GitHub project columns: {}", e),
            })
        })?;

        if !response.status().is_success() {
            return Err(EddaError::Sync(SyncError::Network {
                message: format!(
                    "GitHub API error: {} {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            }));
        }

        let columns: Vec<GitHubProjectColumn> = response.json().await.map_err(|e| {
            EddaError::Sync(SyncError::Network {
                message: format!("Failed to parse GitHub project columns: {}", e),
            })
        })?;

        Ok(columns)
    }

    /// Create a project card
    pub async fn create_project_card(
        &self,
        column_id: u64,
        note: &str,
    ) -> EddaResult<GitHubProjectCard> {
        let url = format!("{}/projects/columns/{}/cards", self.base_url, column_id);

        let payload = serde_json::json!({
            "note": note,
        });

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                EddaError::Sync(SyncError::Network {
                    message: format!("Failed to create GitHub project card: {}", e),
                })
            })?;

        if !response.status().is_success() {
            return Err(EddaError::Sync(SyncError::Network {
                message: format!(
                    "GitHub API error: {} {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            }));
        }

        let card: GitHubProjectCard = response.json().await.map_err(|e| {
            EddaError::Sync(SyncError::Network {
                message: format!("Failed to parse GitHub project card: {}", e),
            })
        })?;

        Ok(card)
    }

    /// Update a project card
    pub async fn update_project_card(
        &self,
        card_id: u64,
        note: &str,
    ) -> EddaResult<GitHubProjectCard> {
        let url = format!("{}/projects/columns/cards/{}", self.base_url, card_id);

        let payload = serde_json::json!({
            "note": note,
        });

        let response = self
            .client
            .patch(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                EddaError::Sync(SyncError::Network {
                    message: format!("Failed to update GitHub project card: {}", e),
                })
            })?;

        if !response.status().is_success() {
            return Err(EddaError::Sync(SyncError::Network {
                message: format!(
                    "GitHub API error: {} {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            }));
        }

        let card: GitHubProjectCard = response.json().await.map_err(|e| {
            EddaError::Sync(SyncError::Network {
                message: format!("Failed to parse GitHub project card: {}", e),
            })
        })?;

        Ok(card)
    }

    /// Move a project card to a different column
    pub async fn move_project_card(
        &self,
        card_id: u64,
        column_id: u64,
        position: Option<&str>,
    ) -> EddaResult<()> {
        let url = format!("{}/projects/columns/cards/{}/moves", self.base_url, card_id);

        let mut payload = serde_json::json!({
            "column_id": column_id,
        });

        if let Some(pos) = position {
            payload["position"] = serde_json::Value::String(pos.to_string());
        }

        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                EddaError::Sync(SyncError::Network {
                    message: format!("Failed to move GitHub project card: {}", e),
                })
            })?;

        if !response.status().is_success() {
            return Err(EddaError::Sync(SyncError::Network {
                message: format!(
                    "GitHub API error: {} {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            }));
        }

        Ok(())
    }

    /// Delete a project card
    pub async fn delete_project_card(&self, card_id: u64) -> EddaResult<()> {
        let url = format!("{}/projects/columns/cards/{}", self.base_url, card_id);

        let response = self.client.delete(&url).send().await.map_err(|e| {
            EddaError::Sync(SyncError::Network {
                message: format!("Failed to delete GitHub project card: {}", e),
            })
        })?;

        if !response.status().is_success() {
            return Err(EddaError::Sync(SyncError::Network {
                message: format!(
                    "GitHub API error: {} {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            }));
        }

        Ok(())
    }

    /// Get all cards from a project
    pub async fn get_project_cards(&self, project_id: u64) -> EddaResult<Vec<GitHubProjectCard>> {
        let columns = self.get_project_columns(project_id).await?;
        let mut all_cards = Vec::new();

        for column in columns {
            let url = format!("{}/projects/columns/{}/cards", self.base_url, column.id);

            let response = self.client.get(&url).send().await.map_err(|e| {
                EddaError::Sync(SyncError::Network {
                    message: format!("Failed to fetch GitHub project cards: {}", e),
                })
            })?;

            if !response.status().is_success() {
                return Err(EddaError::Sync(SyncError::Network {
                    message: format!(
                        "GitHub API error: {} {}",
                        response.status(),
                        response.text().await.unwrap_or_default()
                    ),
                }));
            }

            let cards: Vec<GitHubProjectCard> = response.json().await.map_err(|e| {
                EddaError::Sync(SyncError::Network {
                    message: format!("Failed to parse GitHub project cards: {}", e),
                })
            })?;

            all_cards.extend(cards);
        }

        Ok(all_cards)
    }

    /// Get workflow runs
    pub async fn get_workflow_runs(
        &self,
        workflow_id: Option<u64>,
    ) -> EddaResult<Vec<GitHubWorkflowRun>> {
        let url = if let Some(workflow_id) = workflow_id {
            format!(
                "{}/repos/{}/{}/actions/workflows/{}/runs",
                self.base_url, self.owner, self.repo, workflow_id
            )
        } else {
            format!(
                "{}/repos/{}/{}/actions/runs",
                self.base_url, self.owner, self.repo
            )
        };

        let response = self.client.get(&url).send().await.map_err(|e| {
            EddaError::Sync(SyncError::Network {
                message: format!("Failed to fetch GitHub workflow runs: {}", e),
            })
        })?;

        if !response.status().is_success() {
            return Err(EddaError::Sync(SyncError::Network {
                message: format!(
                    "GitHub API error: {} {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                ),
            }));
        }

        let runs: Vec<GitHubWorkflowRun> = response.json().await.map_err(|e| {
            EddaError::Sync(SyncError::Network {
                message: format!("Failed to parse GitHub workflow runs: {}", e),
            })
        })?;

        Ok(runs)
    }

    /// Convert a GitHub issue to a local task
    pub fn issue_to_task(&self, issue: &GitHubIssue) -> Task {
        let mut task = Task::new(issue.title.clone());

        // Combine title and body for description
        let mut description = issue.title.clone();
        if let Some(body) = &issue.body {
            if !body.is_empty() {
                description.push_str("\n\nGitHub Issue: ");
                description.push_str(body);
            }
        }
        task.description = description;

        // Map issue state to task status
        task.status = match issue.state.as_str() {
            "closed" => crate::core::task::TaskStatus::Completed,
            _ => crate::core::task::TaskStatus::Pending,
        };

        // Add GitHub URL as annotation
        task.add_annotation(format!("GitHub Issue: {}", issue.html_url));

        task
    }

    /// Convert a project card to a local task
    pub fn card_to_task(&self, card: &GitHubProjectCard, column_name: &str) -> Task {
        let mut task = Task::new(card.note.clone().unwrap_or_default());

        // Map column name to task status using config mapping
        let status = match column_name {
            "To Do" => crate::core::task::TaskStatus::Pending,
            "In Progress" => crate::core::task::TaskStatus::InProgress,
            "Done" => crate::core::task::TaskStatus::Completed,
            _ => crate::core::task::TaskStatus::Pending,
        };
        task.status = status;

        // Add project card info as annotation
        task.add_annotation(format!("GitHub Project Card: {}", card.id));

        task
    }

    /// Convert a task to GitHub issue data
    pub fn task_to_issue_data(&self, task: &Task) -> serde_json::Value {
        let mut data = serde_json::Map::new();
        data.insert(
            "title".to_string(),
            serde_json::Value::String(task.description.clone()),
        );

        // Map task status to issue state
        let state = match task.status {
            crate::core::task::TaskStatus::Completed => "closed",
            _ => "open",
        };
        data.insert(
            "state".to_string(),
            serde_json::Value::String(state.to_string()),
        );

        serde_json::Value::Object(data)
    }

    /// Convert a task to project card note
    pub fn task_to_card_note(&self, task: &Task) -> String {
        task.description.clone()
    }

    /// Get the appropriate column ID for a task status
    pub async fn get_column_id_for_status(
        &self,
        project_id: u64,
        status: &crate::core::task::TaskStatus,
        column_mapping: &std::collections::HashMap<String, String>,
    ) -> EddaResult<Option<u64>> {
        let columns = self.get_project_columns(project_id).await?;

        // Find the column name that maps to this status
        let target_status = match status {
            crate::core::task::TaskStatus::Pending => "todo",
            crate::core::task::TaskStatus::InProgress => "in_progress",
            crate::core::task::TaskStatus::Completed => "done",
            crate::core::task::TaskStatus::Deleted => "deleted",
            crate::core::task::TaskStatus::Waiting => "waiting",
        };

        for (column_name, mapped_status) in column_mapping {
            if mapped_status == target_status {
                // Find the column with this name
                for column in &columns {
                    if column.name == *column_name {
                        return Ok(Some(column.id));
                    }
                }
            }
        }

        // Fallback to default mapping
        for column in &columns {
            match (status, column.name.as_str()) {
                (crate::core::task::TaskStatus::Pending, "To Do")
                | (crate::core::task::TaskStatus::InProgress, "In Progress")
                | (crate::core::task::TaskStatus::Completed, "Done")
                | (crate::core::task::TaskStatus::Deleted, "Done")
                | (crate::core::task::TaskStatus::Waiting, "To Do") => {
                    return Ok(Some(column.id));
                }
                _ => {}
            }
        }

        Ok(None)
    }
}

/// GitHub integration manager
pub struct GitHubIntegration {
    client: GitHubClient,
    issue_mapping: HashMap<i64, u64>, // task_id -> issue_number
}

impl GitHubIntegration {
    /// Create a new GitHub integration
    pub fn new(config: GitHubConfig) -> Result<Self, EddaError> {
        Ok(Self {
            client: GitHubClient::new(config)?,
            issue_mapping: HashMap::new(),
        })
    }

    /// Sync local tasks to GitHub issues
    pub async fn sync_tasks_to_github(&mut self, tasks: &[Task]) -> EddaResult<()> {
        for task in tasks {
            if let Some(issue_number) = self.issue_mapping.get(&task.id.unwrap_or(0)) {
                // Update existing issue
                let _issue = self
                    .client
                    .update_issue(
                        *issue_number,
                        Some(&task.description),
                        None,
                        Some(if task.status == crate::core::task::TaskStatus::Completed {
                            "closed"
                        } else {
                            "open"
                        }),
                    )
                    .await?;
            } else {
                // Create new issue
                let issue = self.client.create_issue(&task.description, None).await?;
                if let Some(task_id) = task.id {
                    self.issue_mapping.insert(task_id, issue.number);
                }
            }
        }
        Ok(())
    }

    /// Sync GitHub issues to local tasks
    pub async fn sync_github_to_tasks(&self) -> EddaResult<Vec<Task>> {
        let issues = self.client.get_issues(None).await?;
        let mut tasks = Vec::new();

        for issue in issues {
            let task = self.client.issue_to_task(&issue);
            tasks.push(task);
        }

        Ok(tasks)
    }

    /// Get project board information
    pub async fn get_project_info(&self) -> EddaResult<Vec<GitHubProject>> {
        self.client.get_projects().await
    }

    /// Get workflow run information
    pub async fn get_workflow_info(&self) -> EddaResult<Vec<GitHubWorkflowRun>> {
        self.client.get_workflow_runs(None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::GitHubConfig;

    #[test]
    fn test_github_client_new() {
        let config = GitHubConfig {
            repository: Some("test_owner/test_repo".to_string()),
            sync_interval: 300,
            sync_mode: "issues".to_string(),
            project_ids: vec![1234567890],
            column_mapping: std::collections::HashMap::new(),
        };

        let client = GitHubClient::new(config).unwrap();
        assert_eq!(client.base_url, "https://api.github.com");
        assert_eq!(client.owner, "test_owner");
        assert_eq!(client.repo, "test_repo");
    }

    #[test]
    fn test_github_client_new_missing_token() {
        unsafe {
            std::env::remove_var("GITHUB_TOKEN");
            std::env::remove_var("EDDA_GITHUB_TOKEN");
            std::env::remove_var("GH_TOKEN");
            std::env::remove_var("GITHUB_ACCESS_TOKEN");
        }

        let config = GitHubConfig {
            repository: Some("test_owner/test_repo".to_string()),
            sync_interval: 300,
            sync_mode: "issues".to_string(),
            project_ids: vec![1234567890],
            column_mapping: std::collections::HashMap::new(),
        };

        let result = GitHubClient::new(config);
        assert!(result.is_err());
    }

    #[test]
    fn test_github_client_new_invalid_repository() {
        let config = GitHubConfig {
            repository: Some("invalid_repo_format".to_string()),
            sync_interval: 300,
            sync_mode: "issues".to_string(),
            project_ids: vec![1234567890],
            column_mapping: std::collections::HashMap::new(),
        };

        let result = GitHubClient::new(config);
        assert!(result.is_err());
    }

    #[test]
    fn test_issue_to_task_conversion() {
        unsafe {
            std::env::set_var("GITHUB_TOKEN", "dummy");
        }
        let config = GitHubConfig {
            repository: Some("test_owner/test_repo".to_string()),
            sync_interval: 300,
            sync_mode: "issues".to_string(),
            project_ids: vec![1234567890],
            column_mapping: std::collections::HashMap::new(),
        };

        let client = GitHubClient::new(config).unwrap();

        let issue = GitHubIssue {
            id: 1,
            number: 1,
            title: "Test Issue".to_string(),
            body: Some("Test body".to_string()),
            state: "open".to_string(),
            labels: vec![],
            assignees: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
            closed_at: None,
            milestone: None,
            comments: 0,
            html_url: "https://github.com/test/test/issues/1".to_string(),
        };

        let task = client.issue_to_task(&issue);
        assert_eq!(task.description, "Test Issue\n\nGitHub Issue: Test body");
        assert_eq!(task.status, crate::core::task::TaskStatus::Pending);
    }

    #[test]
    fn test_task_to_issue_data_conversion() {
        unsafe {
            std::env::set_var("GITHUB_TOKEN", "dummy");
        }
        let config = GitHubConfig {
            repository: Some("test_owner/test_repo".to_string()),
            sync_interval: 300,
            sync_mode: "issues".to_string(),
            project_ids: vec![1234567890],
            column_mapping: std::collections::HashMap::new(),
        };

        let client = GitHubClient::new(config).unwrap();

        let mut task = Task::new("Test Task".to_string());
        task.status = crate::core::task::TaskStatus::Completed;

        let issue_data = client.task_to_issue_data(&task);
        let data = issue_data.as_object().unwrap();

        assert_eq!(data.get("title").unwrap().as_str().unwrap(), "Test Task");
        assert_eq!(data.get("state").unwrap().as_str().unwrap(), "closed");
    }
}

/// GitHub sync provider implementation
pub struct GitHubSyncProvider {
    client: GitHubClient,
    config: crate::core::config::GitHubConfig,
    issue_mapping: HashMap<i64, u64>, // task_id -> issue_number
    card_mapping: HashMap<i64, u64>,  // task_id -> card_id
}

impl GitHubSyncProvider {
    /// Create a new GitHub sync provider
    pub fn new(config: crate::core::config::GitHubConfig) -> Result<Self, EddaError> {
        Ok(Self {
            client: GitHubClient::new(config.clone())?,
            config,
            issue_mapping: HashMap::new(),
            card_mapping: HashMap::new(),
        })
    }
}

#[async_trait]
impl SyncProvider for GitHubSyncProvider {
    fn name(&self) -> &str {
        "GitHub"
    }

    async fn pull_tasks(&self) -> EddaResult<Vec<Task>> {
        let mut all_tasks = Vec::new();

        match self.config.sync_mode.as_str() {
            "issues" => {
                let issues = self.client.get_issues(None).await?;
                for issue in issues {
                    let task = self.client.issue_to_task(&issue);
                    all_tasks.push(task);
                }
            }
            "projects" => {
                for project_id in &self.config.project_ids {
                    let cards = self.client.get_project_cards(*project_id).await?;
                    let columns = self.client.get_project_columns(*project_id).await?;
                    let mut column_names = HashMap::new();
                    for column in columns {
                        column_names.insert(column.id, column.name);
                    }
                    let unknown = "Unknown".to_string();
                    for card in cards {
                        let column_name = column_names
                            .get(&card.column_id)
                            .unwrap_or(&unknown)
                            .clone();
                        let task = self.client.card_to_task(&card, &column_name);
                        all_tasks.push(task);
                    }
                }
            }
            "both" => {
                let issues = self.client.get_issues(None).await?;
                for issue in issues {
                    let task = self.client.issue_to_task(&issue);
                    all_tasks.push(task);
                }
                for project_id in &self.config.project_ids {
                    let cards = self.client.get_project_cards(*project_id).await?;
                    let columns = self.client.get_project_columns(*project_id).await?;
                    let mut column_names = HashMap::new();
                    for column in columns {
                        column_names.insert(column.id, column.name);
                    }
                    let unknown = "Unknown".to_string();
                    for card in cards {
                        let column_name = column_names
                            .get(&card.column_id)
                            .unwrap_or(&unknown)
                            .clone();
                        let task = self.client.card_to_task(&card, &column_name);
                        all_tasks.push(task);
                    }
                }
            }
            _ => {
                return Err(EddaError::Sync(SyncError::Configuration {
                    message: format!("Invalid sync_mode: {}", self.config.sync_mode),
                }));
            }
        }

        Ok(all_tasks)
    }

    async fn push_tasks(&self, tasks: &[Task]) -> EddaResult<()> {
        match self.config.sync_mode.as_str() {
            "issues" => {
                for task in tasks {
                    if let Some(issue_number) = self.issue_mapping.get(&task.id.unwrap_or(0)) {
                        let _issue = self
                            .client
                            .update_issue(
                                *issue_number,
                                Some(&task.description),
                                None,
                                Some(if task.status == crate::core::task::TaskStatus::Completed {
                                    "closed"
                                } else {
                                    "open"
                                }),
                            )
                            .await?;
                    } else {
                        let _issue = self.client.create_issue(&task.description, None).await?;
                    }
                }
            }
            "projects" => {
                for project_id in &self.config.project_ids {
                    for task in tasks {
                        if let Some(card_id) = self.card_mapping.get(&task.id.unwrap_or(0)) {
                            let _card = self
                                .client
                                .update_project_card(*card_id, &task.description)
                                .await?;
                            if let Some(column_id) = self
                                .client
                                .get_column_id_for_status(
                                    *project_id,
                                    &task.status,
                                    &self.config.column_mapping,
                                )
                                .await?
                            {
                                self.client
                                    .move_project_card(*card_id, column_id, None)
                                    .await?;
                            }
                        } else {
                            if let Some(column_id) = self
                                .client
                                .get_column_id_for_status(
                                    *project_id,
                                    &task.status,
                                    &self.config.column_mapping,
                                )
                                .await?
                            {
                                let _card = self
                                    .client
                                    .create_project_card(column_id, &task.description)
                                    .await?;
                            }
                        }
                    }
                }
            }
            "both" => {
                for task in tasks {
                    if let Some(issue_number) = self.issue_mapping.get(&task.id.unwrap_or(0)) {
                        let _issue = self
                            .client
                            .update_issue(
                                *issue_number,
                                Some(&task.description),
                                None,
                                Some(if task.status == crate::core::task::TaskStatus::Completed {
                                    "closed"
                                } else {
                                    "open"
                                }),
                            )
                            .await?;
                    } else {
                        let _issue = self.client.create_issue(&task.description, None).await?;
                    }
                }
                for project_id in &self.config.project_ids {
                    for task in tasks {
                        if let Some(card_id) = self.card_mapping.get(&task.id.unwrap_or(0)) {
                            let _card = self
                                .client
                                .update_project_card(*card_id, &task.description)
                                .await?;
                            if let Some(column_id) = self
                                .client
                                .get_column_id_for_status(
                                    *project_id,
                                    &task.status,
                                    &self.config.column_mapping,
                                )
                                .await?
                            {
                                self.client
                                    .move_project_card(*card_id, column_id, None)
                                    .await?;
                            }
                        } else {
                            if let Some(column_id) = self
                                .client
                                .get_column_id_for_status(
                                    *project_id,
                                    &task.status,
                                    &self.config.column_mapping,
                                )
                                .await?
                            {
                                let _card = self
                                    .client
                                    .create_project_card(column_id, &task.description)
                                    .await?;
                            }
                        }
                    }
                }
            }
            _ => {
                return Err(EddaError::Sync(SyncError::Configuration {
                    message: format!("Invalid sync_mode: {}", self.config.sync_mode),
                }));
            }
        }
        Ok(())
    }

    async fn get_status(&self) -> EddaResult<SyncStatus> {
        match self.config.sync_mode.as_str() {
            "issues" => {
                // Test connection by trying to get issues
                match self.client.get_issues(Some("open")).await {
                    Ok(_) => Ok(SyncStatus::Completed),
                    Err(_) => Ok(SyncStatus::Failed {
                        error: "Failed to connect to GitHub Issues".to_string(),
                    }),
                }
            }
            "projects" => {
                // Test connection by trying to get all projects
                if self.config.project_ids.is_empty() {
                    return Ok(SyncStatus::Failed {
                        error: "No project_ids configured".to_string(),
                    });
                }
                for project_id in &self.config.project_ids {
                    if self.client.get_project_columns(*project_id).await.is_err() {
                        return Ok(SyncStatus::Failed {
                            error: format!("Failed to connect to GitHub Project ID {}", project_id),
                        });
                    }
                }
                Ok(SyncStatus::Completed)
            }
            "both" => {
                let issues_ok = self.client.get_issues(Some("open")).await.is_ok();
                let mut projects_ok = true;
                if self.config.project_ids.is_empty() {
                    projects_ok = false;
                } else {
                    for project_id in &self.config.project_ids {
                        if self.client.get_project_columns(*project_id).await.is_err() {
                            projects_ok = false;
                            break;
                        }
                    }
                }
                if issues_ok && projects_ok {
                    Ok(SyncStatus::Completed)
                } else {
                    Ok(SyncStatus::Failed {
                        error: "Failed to connect to one or more GitHub services".to_string(),
                    })
                }
            }
            _ => Ok(SyncStatus::Failed {
                error: format!("Invalid sync_mode: {}", self.config.sync_mode),
            }),
        }
    }

    async fn test_connection(&self) -> EddaResult<()> {
        match self.config.sync_mode.as_str() {
            "issues" => {
                self.client.get_issues(Some("open")).await?;
            }
            "projects" => {
                if self.config.project_ids.is_empty() {
                    return Err(EddaError::Sync(SyncError::Configuration {
                        message: "project_ids is required for projects sync mode".to_string(),
                    }));
                }
                for project_id in &self.config.project_ids {
                    self.client.get_project_columns(*project_id).await?;
                }
            }
            "both" => {
                self.client.get_issues(Some("open")).await?;
                if self.config.project_ids.is_empty() {
                    return Err(EddaError::Sync(SyncError::Configuration {
                        message: "project_ids is required for both sync mode".to_string(),
                    }));
                }
                for project_id in &self.config.project_ids {
                    self.client.get_project_columns(*project_id).await?;
                }
            }
            _ => {
                return Err(EddaError::Sync(SyncError::Configuration {
                    message: format!("Invalid sync_mode: {}", self.config.sync_mode),
                }));
            }
        }
        Ok(())
    }
}
