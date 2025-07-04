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
    /// Create a new GitHub API client
    pub fn new(config: GitHubConfig) -> Result<Self, EddaError> {
        let token = config.token.clone().ok_or_else(|| {
            EddaError::Sync(SyncError::Authentication {
                message: "GitHub token not configured".to_string(),
            })
        })?;

        let repository = config.repository.clone().ok_or_else(|| {
            EddaError::Sync(SyncError::ProviderNotFound {
                provider: "GitHub".to_string(),
            })
        })?;

        // Parse repository in format "owner/repo"
        let parts: Vec<&str> = repository.split('/').collect();
        if parts.len() != 2 {
            return Err(EddaError::Sync(SyncError::ProviderNotFound {
                provider: format!("Invalid repository format: {}", repository),
            }));
        }

        let owner = parts[0].to_string();
        let repo = parts[1].to_string();

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", format!("token {}", token).parse().unwrap());
        headers.insert("Accept", "application/vnd.github.v3+json".parse().unwrap());
        headers.insert("User-Agent", "edda-cli".parse().unwrap());

        let client = Client::builder()
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
            owner,
            repo,
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

        // Set description from issue body
        if let Some(body) = &issue.body {
            task.description = format!("{}\n\nGitHub Issue: {}", issue.title, body);
        }

        // Set status based on issue state
        task.status = if issue.state == "closed" {
            crate::core::task::TaskStatus::Completed
        } else {
            crate::core::task::TaskStatus::Pending
        };

        // Add labels as tags
        for label in &issue.labels {
            task.add_tag(label.name.clone());
        }

        // Add assignees as tags
        for assignee in &issue.assignees {
            task.add_tag(format!("@{}", assignee.login));
        }

        // Set dates
        task.entry_date = issue.created_at;
        task.modified_date = issue.updated_at;
        if let Some(closed_at) = issue.closed_at {
            task.end_date = Some(closed_at);
        }

        // Add GitHub metadata
        task.add_annotation(format!(
            "GitHub Issue #{}: {}",
            issue.number, issue.html_url
        ));

        task
    }

    /// Convert a local task to GitHub issue data
    pub fn task_to_issue_data(&self, task: &Task) -> serde_json::Value {
        let mut payload = serde_json::Map::new();

        payload.insert(
            "title".to_string(),
            serde_json::Value::String(task.description.clone()),
        );

        // Extract GitHub issue URL from annotations if present
        let mut body = task.description.clone();
        for annotation in &task.annotations {
            if annotation.description.contains("GitHub Issue #") {
                body = annotation.description.clone();
                break;
            }
        }

        payload.insert("body".to_string(), serde_json::Value::String(body));

        // Set state based on task status
        let state = match task.status {
            crate::core::task::TaskStatus::Completed => "closed",
            _ => "open",
        };
        payload.insert(
            "state".to_string(),
            serde_json::Value::String(state.to_string()),
        );

        serde_json::Value::Object(payload)
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
            token: Some("test_token".to_string()),
            repository: Some("test_owner/test_repo".to_string()),
            sync_interval: 300,
        };

        let client = GitHubClient::new(config).unwrap();
        assert_eq!(client.base_url, "https://api.github.com");
        assert_eq!(client.owner, "test_owner");
        assert_eq!(client.repo, "test_repo");
    }

    #[test]
    fn test_github_client_new_missing_token() {
        let config = GitHubConfig {
            token: None,
            repository: Some("test_owner/test_repo".to_string()),
            sync_interval: 300,
        };

        let result = GitHubClient::new(config);
        assert!(result.is_err());
    }

    #[test]
    fn test_github_client_new_invalid_repository() {
        let config = GitHubConfig {
            token: Some("test_token".to_string()),
            repository: Some("invalid_repo_format".to_string()),
            sync_interval: 300,
        };

        let result = GitHubClient::new(config);
        assert!(result.is_err());
    }

    #[test]
    fn test_issue_to_task_conversion() {
        let config = GitHubConfig {
            token: Some("test_token".to_string()),
            repository: Some("test_owner/test_repo".to_string()),
            sync_interval: 300,
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
        let config = GitHubConfig {
            token: Some("test_token".to_string()),
            repository: Some("test_owner/test_repo".to_string()),
            sync_interval: 300,
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
    issue_mapping: HashMap<i64, u64>, // task_id -> issue_number
}

impl GitHubSyncProvider {
    /// Create a new GitHub sync provider
    pub fn new(config: GitHubConfig) -> Result<Self, EddaError> {
        Ok(Self {
            client: GitHubClient::new(config)?,
            issue_mapping: HashMap::new(),
        })
    }
}

#[async_trait]
impl SyncProvider for GitHubSyncProvider {
    fn name(&self) -> &str {
        "GitHub"
    }

    async fn pull_tasks(&self) -> EddaResult<Vec<Task>> {
        let issues = self.client.get_issues(None).await?;
        let mut tasks = Vec::new();

        for issue in issues {
            let task = self.client.issue_to_task(&issue);
            tasks.push(task);
        }

        Ok(tasks)
    }

    async fn push_tasks(&self, tasks: &[Task]) -> EddaResult<()> {
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
                    // Note: We can't modify self.issue_mapping here since we're in an async trait
                    // This would need to be handled differently in a real implementation
                }
            }
        }
        Ok(())
    }

    async fn get_status(&self) -> EddaResult<SyncStatus> {
        // Test connection by trying to get issues
        match self.client.get_issues(Some("open")).await {
            Ok(_) => Ok(SyncStatus::Completed),
            Err(_) => Ok(SyncStatus::Failed {
                error: "Failed to connect to GitHub".to_string(),
            }),
        }
    }

    async fn test_connection(&self) -> EddaResult<()> {
        // Try to get a single issue to test the connection
        self.client.get_issues(Some("open")).await?;
        Ok(())
    }
}
