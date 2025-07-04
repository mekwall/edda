# Edda - Task Management Specification

## Overview

This specification defines the task management system for Edda, providing comprehensive task tracking, organization, and workflow management capabilities for AI agents. The task management system supports hierarchical task structures, priority management, and integration with AI agent workflows.

## Architecture Context

This component operates within the Edda architecture as defined in [SPEC_ARCHITECTURE.md](./SPEC_ARCHITECTURE.md):

- **Layer**: Core Engine Layer
- **Component**: Task Manager
- **Responsibilities**: Execute business logic for task operations, manage task relationships and constraints, ensure data consistency and integrity, handle task lifecycle management
- **Dependencies**: Data Storage Layer (for task persistence), CLI Interface Layer (for task commands), AI Agent Integration Layer (for AI agent task interactions)

## Architecture Overview

### Task Management Design

The task management system implements the Core Engine Layer responsibilities defined in the master architecture:

- **Task Creator**: Create and validate new tasks
- **Task Engine**: Execute business logic for task operations
- **Workflow Manager**: Manage task workflows and state transitions
- **Dependency Manager**: Handle task dependencies and relationships
- **Project Manager**: Organize tasks into projects and hierarchies
- **Tag Manager**: Manage task categorization and tagging
- **Filter Manager**: Provide task filtering and search capabilities

## Core Task Components

### Task Engine

```rust
pub struct TaskEngine {
    /// Task storage
    storage: Arc<dyn TaskStorage>,
    /// Task validator
    validator: Box<dyn TaskValidator>,
    /// Task processor
    processor: Box<dyn TaskProcessor>,
    /// Workflow manager
    workflow_manager: Arc<WorkflowManager>,
    /// Dependency manager
    dependency_manager: Arc<DependencyManager>,
}

impl TaskEngine {
    /// Create a new task
    pub fn create_task(&self, task_data: &TaskData) -> Result<Task, TaskError> {
        // Validate task data
        self.validator.validate_task_data(task_data)?;

        // Create task
        let task = Task {
            id: TaskId(Uuid::new_v4().to_string()),
            title: task_data.title.clone(),
            description: task_data.description.clone(),
            status: TaskStatus::Pending,
            priority: task_data.priority,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            completed_at: None,
            due_date: task_data.due_date,
            tags: task_data.tags.clone(),
            parent_id: task_data.parent_id.clone(),
            children: Vec::new(),
            documents: Vec::new(),
            metadata: task_data.metadata.clone(),
            agent_context: task_data.agent_context.clone(),
        };

        // Store task
        self.storage.store_task(&task)?;

        // Process task
        self.processor.process_task(&task)?;

        // Update dependencies
        self.dependency_manager.update_dependencies(&task)?;

        Ok(task)
    }

    /// Update an existing task
    pub fn update_task(&self, task_id: &TaskId, updates: &TaskUpdates) -> Result<Task, TaskError> {
        // Get existing task
        let mut task = self.storage.get_task(task_id)?
            .ok_or(TaskError::TaskNotFound)?;

        // Apply updates
        if let Some(title) = &updates.title {
            task.title = title.clone();
        }
        if let Some(description) = &updates.description {
            task.description = description.clone();
        }
        if let Some(status) = &updates.status {
            task.status = status.clone();
        }
        if let Some(priority) = &updates.priority {
            task.priority = priority.clone();
        }
        if let Some(due_date) = &updates.due_date {
            task.due_date = *due_date;
        }
        if let Some(tags) = &updates.tags {
            task.tags = tags.clone();
        }

        // Update timestamps
        task.updated_at = Utc::now();

        // Handle status changes
        if let Some(status) = &updates.status {
            self.handle_status_change(&mut task, status)?;
        }

        // Store updated task
        self.storage.update_task(&task)?;

        // Process updates
        self.processor.process_task_update(&task, updates)?;

        Ok(task)
    }

    /// Delete a task
    pub fn delete_task(&self, task_id: &TaskId) -> Result<(), TaskError> {
        // Check if task has children
        let task = self.storage.get_task(task_id)?
            .ok_or(TaskError::TaskNotFound)?;

        if !task.children.is_empty() {
            return Err(TaskError::TaskHasChildren);
        }

        // Remove from parent
        if let Some(parent_id) = &task.parent_id {
            self.remove_child_from_parent(parent_id, task_id)?;
        }

        // Delete task
        self.storage.delete_task(task_id)?;

        // Update dependencies
        self.dependency_manager.remove_task_dependencies(task_id)?;

        Ok(())
    }

    /// Get task by ID
    pub fn get_task(&self, task_id: &TaskId) -> Result<Option<Task>, TaskError> {
        self.storage.get_task(task_id)
    }

    /// Query tasks
    pub fn query_tasks(&self, query: &TaskQuery) -> Result<Vec<Task>, TaskError> {
        self.storage.query_tasks(query)
    }

    /// Handle status change
    fn handle_status_change(&self, task: &mut Task, new_status: &TaskStatus) -> Result<(), TaskError> {
        match new_status {
            TaskStatus::Completed => {
                task.end_date = Some(Utc::now());
                self.workflow_manager.handle_task_completion(task)?;
            }
            TaskStatus::Deleted => {
                self.workflow_manager.handle_task_deletion(task)?;
            }
            TaskStatus::Waiting => {
                self.workflow_manager.handle_task_waiting(task)?;
            }
            _ => {}
        }

        Ok(())
    }
}
```

## Task Data Models

### Task Data

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskData {
    /// Task title
    pub title: String,
    /// Task description
    pub description: Option<String>,
    /// Task priority
    pub priority: Priority,
    /// Due date
    pub due_date: Option<DateTime<Utc>>,
    /// Parent task ID
    pub parent_id: Option<TaskId>,
    /// Task tags
    pub tags: Vec<String>,
    /// Custom metadata
    pub metadata: HashMap<String, Value>,
    /// AI agent context
    pub agent_context: Option<AgentContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskUpdates {
    /// Updated title
    pub title: Option<String>,
    /// Updated description
    pub description: Option<String>,
    /// Updated status
    pub status: Option<TaskStatus>,
    /// Updated priority
    pub priority: Option<Priority>,
    /// Updated due date
    pub due_date: Option<DateTime<Utc>>,
    /// Updated tags
    pub tags: Option<Vec<String>>,
    /// Updated metadata
    pub metadata: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskQuery {
    /// Status filter
    pub status: Option<TaskStatus>,
    /// Priority filter
    pub priority: Option<Priority>,
    /// Tag filter
    pub tags: Option<Vec<String>>,
    /// Date range filter
    pub date_range: Option<DateRange>,
    /// Parent task filter
    pub parent_id: Option<TaskId>,
    /// Agent context filter
    pub agent_context: Option<AgentContext>,
    /// Text search
    pub search: Option<String>,
    /// Sort criteria
    pub sort: Vec<SortCriteria>,
    /// Pagination
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    /// Start date
    pub start: DateTime<Utc>,
    /// End date
    pub end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortCriteria {
    /// Field to sort by
    pub field: String,
    /// Sort direction
    pub direction: SortDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortDirection {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    /// Page number
    pub page: u64,
    /// Page size
    pub page_size: u64,
}
```

## Priority Management

### Priority Manager

```rust
pub struct PriorityManager {
    /// Priority levels
    levels: Vec<PriorityLevel>,
    /// Priority rules
    rules: Vec<Box<dyn PriorityRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityLevel {
    /// Priority value
    pub value: Priority,
    /// Priority name
    pub name: String,
    /// Priority color
    pub color: String,
    /// Priority description
    pub description: String,
}

impl PriorityManager {
    /// Get all priority levels
    pub fn get_priority_levels(&self) -> Vec<PriorityLevel> {
        self.levels.clone()
    }

    /// Get priority level by value
    pub fn get_priority_level(&self, priority: Priority) -> Option<PriorityLevel> {
        self.levels.iter()
            .find(|level| level.value == priority)
            .cloned()
    }

    /// Suggest priority for task
    pub fn suggest_priority(&self, task_data: &TaskData) -> Result<Priority, PriorityError> {
        let mut suggestions = Vec::new();

        // Apply priority rules
        for rule in &self.rules {
            if let Some(suggestion) = rule.suggest_priority(task_data)? {
                suggestions.push(suggestion);
            }
        }

        // Aggregate suggestions
        if suggestions.is_empty() {
            Ok(Priority::Medium)
        } else {
            Ok(self.aggregate_suggestions(&suggestions)?)
        }
    }

    /// Aggregate priority suggestions
    fn aggregate_suggestions(&self, suggestions: &[Priority]) -> Result<Priority, PriorityError> {
        // Calculate weighted average
        let total_weight: u32 = suggestions.iter().map(|p| *p as u32).sum();
        let avg_value = total_weight / suggestions.len() as u32;

        // Convert back to Priority
        match avg_value {
            1 => Ok(Priority::Low),
            2 => Ok(Priority::Medium),
            3 => Ok(Priority::High),
            _ => Ok(Priority::Medium),
        }
    }
}
```

## Status Management

### Status Manager

```rust
pub struct StatusManager {
    /// Status transitions
    transitions: HashMap<TaskStatus, Vec<TaskStatus>>,
    /// Status rules
    rules: Vec<Box<dyn StatusRule>>,
}

impl StatusManager {
    /// Get valid status transitions
    pub fn get_valid_transitions(&self, current_status: TaskStatus) -> Vec<TaskStatus> {
        self.transitions.get(&current_status)
            .cloned()
            .unwrap_or_default()
    }

    /// Validate status transition
    pub fn validate_transition(&self, from: TaskStatus, to: TaskStatus) -> Result<bool, StatusError> {
        let valid_transitions = self.get_valid_transitions(from);
        Ok(valid_transitions.contains(&to))
    }

    /// Apply status rules
    pub fn apply_status_rules(&self, task: &Task, new_status: TaskStatus) -> Result<(), StatusError> {
        for rule in &self.rules {
            rule.apply(task, new_status)?;
        }
        Ok(())
    }

    /// Get status statistics
    pub fn get_status_statistics(&self, tasks: &[Task]) -> HashMap<TaskStatus, u64> {
        let mut stats = HashMap::new();

        for task in tasks {
            *stats.entry(task.status.clone()).or_insert(0) += 1;
        }

        stats
    }
}
```

## Workflow Management

### Workflow Manager

```rust
pub struct WorkflowManager {
    /// Workflow definitions
    workflows: HashMap<String, WorkflowDefinition>,
    /// Active workflows
    active_workflows: HashMap<String, ActiveWorkflow>,
    /// Workflow engine
    engine: Box<dyn WorkflowEngine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDefinition {
    /// Workflow ID
    pub id: String,
    /// Workflow name
    pub name: String,
    /// Workflow steps
    pub steps: Vec<WorkflowStep>,
    /// Workflow triggers
    pub triggers: Vec<WorkflowTrigger>,
    /// Workflow conditions
    pub conditions: Vec<WorkflowCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    /// Step ID
    pub id: String,
    /// Step name
    pub name: String,
    /// Step type
    pub step_type: WorkflowStepType,
    /// Step configuration
    pub config: HashMap<String, Value>,
    /// Next steps
    pub next_steps: Vec<String>,
    /// Error handling
    pub error_handling: ErrorHandling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowStepType {
    /// Create task
    CreateTask,
    /// Update task
    UpdateTask,
    /// Send notification
    SendNotification,
    /// Execute command
    ExecuteCommand,
    /// Wait for condition
    WaitForCondition,
    /// Custom step
    Custom(String),
}

impl WorkflowManager {
    /// Register workflow
    pub fn register_workflow(&mut self, workflow: WorkflowDefinition) -> Result<(), WorkflowError> {
        // Validate workflow
        self.validate_workflow(&workflow)?;

        // Register workflow
        self.workflows.insert(workflow.id.clone(), workflow);

        Ok(())
    }

    /// Start workflow
    pub fn start_workflow(&mut self, workflow_id: &str, context: &WorkflowContext) -> Result<ActiveWorkflow, WorkflowError> {
        let workflow = self.workflows.get(workflow_id)
            .ok_or(WorkflowError::WorkflowNotFound)?;

        // Create active workflow
        let active_workflow = ActiveWorkflow {
            id: Uuid::new_v4().to_string(),
            workflow_id: workflow_id.to_string(),
            current_step: workflow.steps.first().map(|s| s.id.clone()),
            context: context.clone(),
            started_at: Utc::now(),
            completed_at: None,
            status: WorkflowStatus::Running,
        };

        // Start execution
        self.engine.start_workflow(&active_workflow)?;

        // Store active workflow
        self.active_workflows.insert(active_workflow.id.clone(), active_workflow.clone());

        Ok(active_workflow)
    }

    /// Handle task completion
    pub fn handle_task_completion(&self, task: &Task) -> Result<(), WorkflowError> {
        // Find workflows triggered by task completion
        let triggered_workflows = self.find_triggered_workflows(task, WorkflowTriggerType::TaskCompleted)?;

        // Start triggered workflows
        for workflow in triggered_workflows {
            let context = WorkflowContext {
                task_id: Some(task.id.clone()),
                agent_context: task.agent_context.clone(),
                metadata: task.metadata.clone(),
            };

            self.start_workflow(&workflow.id, &context)?;
        }

        Ok(())
    }

    /// Validate workflow
    fn validate_workflow(&self, workflow: &WorkflowDefinition) -> Result<(), WorkflowError> {
        // Check for cycles
        self.check_for_cycles(workflow)?;

        // Validate steps
        for step in &workflow.steps {
            self.validate_step(step)?;
        }

        // Validate triggers
        for trigger in &workflow.triggers {
            self.validate_trigger(trigger)?;
        }

        Ok(())
    }
}
```

## Dependency Management

### Dependency Manager

```rust
pub struct DependencyManager {
    /// Dependency graph
    dependency_graph: Arc<RwLock<DependencyGraph>>,
    /// Dependency rules
    rules: Vec<Box<dyn DependencyRule>>,
}

#[derive(Debug, Clone)]
pub struct DependencyGraph {
    /// Dependencies by task ID
    dependencies: HashMap<TaskId, Vec<TaskId>>,
    /// Dependents by task ID
    dependents: HashMap<TaskId, Vec<TaskId>>,
}

impl DependencyManager {
    /// Add dependency
    pub fn add_dependency(&self, task_id: &TaskId, depends_on: &TaskId) -> Result<(), DependencyError> {
        // Check for cycles
        if self.would_create_cycle(task_id, depends_on)? {
            return Err(DependencyError::CircularDependency);
        }

        // Add dependency
        {
            let mut graph = self.dependency_graph.write()?;
            graph.dependencies.entry(task_id.clone())
                .or_insert_with(Vec::new)
                .push(depends_on.clone());
            graph.dependents.entry(depends_on.clone())
                .or_insert_with(Vec::new)
                .push(task_id.clone());
        }

        Ok(())
    }

    /// Remove dependency
    pub fn remove_dependency(&self, task_id: &TaskId, depends_on: &TaskId) -> Result<(), DependencyError> {
        {
            let mut graph = self.dependency_graph.write()?;

            // Remove from dependencies
            if let Some(deps) = graph.dependencies.get_mut(task_id) {
                deps.retain(|id| id != depends_on);
            }

            // Remove from dependents
            if let Some(deps) = graph.dependents.get_mut(depends_on) {
                deps.retain(|id| id != task_id);
            }
        }

        Ok(())
    }

    /// Get dependencies for task
    pub fn get_dependencies(&self, task_id: &TaskId) -> Result<Vec<TaskId>, DependencyError> {
        let graph = self.dependency_graph.read()?;
        Ok(graph.dependencies.get(task_id).cloned().unwrap_or_default())
    }

    /// Get dependents for task
    pub fn get_dependents(&self, task_id: &TaskId) -> Result<Vec<TaskId>, DependencyError> {
        let graph = self.dependency_graph.read()?;
        Ok(graph.dependents.get(task_id).cloned().unwrap_or_default())
    }

    /// Check if task can be started
    pub fn can_start_task(&self, task_id: &TaskId) -> Result<bool, DependencyError> {
        let dependencies = self.get_dependencies(task_id)?;

        // Check if all dependencies are completed
        for dep_id in dependencies {
            // This would require access to task storage to check status
            // For now, assume all dependencies are completed
            // In a real implementation, you would check the actual task status
        }

        Ok(true)
    }

    /// Check for cycles
    fn would_create_cycle(&self, task_id: &TaskId, depends_on: &TaskId) -> Result<bool, DependencyError> {
        // Simple cycle detection - in a real implementation, you'd use a more sophisticated algorithm
        if task_id == depends_on {
            return Ok(true);
        }

        // Check if depends_on depends on task_id (directly or indirectly)
        let dependents = self.get_dependents(depends_on)?;
        for dependent in dependents {
            if &dependent == task_id {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
```

## Project Management

### Project Manager

```rust
pub struct ProjectManager {
    /// Project storage
    storage: Arc<dyn ProjectStorage>,
    /// Project templates
    templates: HashMap<String, ProjectTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Project ID
    pub id: ProjectId,
    /// Project name
    pub name: String,
    /// Project description
    pub description: Option<String>,
    /// Project status
    pub status: ProjectStatus,
    /// Project tasks
    pub tasks: Vec<TaskId>,
    /// Project tags
    pub tags: Vec<String>,
    /// Project metadata
    pub metadata: HashMap<String, Value>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectStatus {
    /// Project is active
    Active,
    /// Project is completed
    Completed,
    /// Project is on hold
    OnHold,
    /// Project is cancelled
    Cancelled,
}

impl ProjectManager {
    /// Create project
    pub fn create_project(&self, project_data: &ProjectData) -> Result<Project, ProjectError> {
        let project = Project {
            id: ProjectId(Uuid::new_v4().to_string()),
            name: project_data.name.clone(),
            description: project_data.description.clone(),
            status: ProjectStatus::Active,
            tasks: Vec::new(),
            tags: project_data.tags.clone(),
            metadata: project_data.metadata.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.storage.store_project(&project)?;
        Ok(project)
    }

    /// Add task to project
    pub fn add_task_to_project(&self, project_id: &ProjectId, task_id: &TaskId) -> Result<(), ProjectError> {
        let mut project = self.storage.get_project(project_id)?
            .ok_or(ProjectError::ProjectNotFound)?;

        if !project.tasks.contains(task_id) {
            project.tasks.push(task_id.clone());
            project.updated_at = Utc::now();
            self.storage.update_project(&project)?;
        }

        Ok(())
    }

    /// Remove task from project
    pub fn remove_task_from_project(&self, project_id: &ProjectId, task_id: &TaskId) -> Result<(), ProjectError> {
        let mut project = self.storage.get_project(project_id)?
            .ok_or(ProjectError::ProjectNotFound)?;

        project.tasks.retain(|id| id != task_id);
        project.updated_at = Utc::now();
        self.storage.update_project(&project)?;

        Ok(())
    }

    /// Get project tasks
    pub fn get_project_tasks(&self, project_id: &ProjectId) -> Result<Vec<Task>, ProjectError> {
        let project = self.storage.get_project(project_id)?
            .ok_or(ProjectError::ProjectNotFound)?;

        let mut tasks = Vec::new();
        for task_id in &project.tasks {
            if let Some(task) = self.storage.get_task(task_id)? {
                tasks.push(task);
            }
        }

        Ok(tasks)
    }
}
```

## Tag Management

### Tag Manager

```rust
pub struct TagManager {
    /// Tag storage
    storage: Arc<dyn TagStorage>,
    /// Tag statistics
    statistics: Arc<RwLock<TagStatistics>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// Tag ID
    pub id: TagId,
    /// Tag name
    pub name: String,
    /// Tag color
    pub color: String,
    /// Tag description
    pub description: Option<String>,
    /// Tag category
    pub category: Option<String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Usage count
    pub usage_count: u64,
}

#[derive(Debug, Clone)]
pub struct TagStatistics {
    /// Tag usage counts
    usage_counts: HashMap<String, u64>,
    /// Tag combinations
    combinations: HashMap<Vec<String>, u64>,
    /// Tag trends
    trends: HashMap<String, Vec<TagTrend>>,
}

impl TagManager {
    /// Create tag
    pub fn create_tag(&self, tag_data: &TagData) -> Result<Tag, TagError> {
        let tag = Tag {
            id: TagId(Uuid::new_v4().to_string()),
            name: tag_data.name.clone(),
            color: tag_data.color.clone(),
            description: tag_data.description.clone(),
            category: tag_data.category.clone(),
            created_at: Utc::now(),
            usage_count: 0,
        };

        self.storage.store_tag(&tag)?;
        Ok(tag)
    }

    /// Get tag by name
    pub fn get_tag_by_name(&self, name: &str) -> Result<Option<Tag>, TagError> {
        self.storage.get_tag_by_name(name)
    }

    /// Get all tags
    pub fn get_all_tags(&self) -> Result<Vec<Tag>, TagError> {
        self.storage.get_all_tags()
    }

    /// Get tags by category
    pub fn get_tags_by_category(&self, category: &str) -> Result<Vec<Tag>, TagError> {
        self.storage.get_tags_by_category(category)
    }

    /// Update tag usage
    pub fn update_tag_usage(&self, tag_name: &str) -> Result<(), TagError> {
        // Update tag usage count
        if let Some(mut tag) = self.storage.get_tag_by_name(tag_name)? {
            tag.usage_count += 1;
            self.storage.update_tag(&tag)?;
        }

        // Update statistics
        {
            let mut stats = self.statistics.write()?;
            *stats.usage_counts.entry(tag_name.to_string()).or_insert(0) += 1;
        }

        Ok(())
    }

    /// Get tag suggestions
    pub fn get_tag_suggestions(&self, partial_name: &str) -> Result<Vec<Tag>, TagError> {
        let all_tags = self.get_all_tags()?;

        let suggestions = all_tags.into_iter()
            .filter(|tag| tag.name.to_lowercase().contains(&partial_name.to_lowercase()))
            .collect();

        Ok(suggestions)
    }

    /// Get popular tags
    pub fn get_popular_tags(&self, limit: usize) -> Result<Vec<Tag>, TagError> {
        let all_tags = self.get_all_tags()?;

        let mut sorted_tags = all_tags;
        sorted_tags.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));

        Ok(sorted_tags.into_iter().take(limit).collect())
    }
}
```

## Filter Management

### Filter Manager

```rust
pub struct FilterManager {
    /// Filter definitions
    filters: HashMap<String, FilterDefinition>,
    /// Filter engine
    engine: Box<dyn FilterEngine>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterDefinition {
    /// Filter ID
    pub id: String,
    /// Filter name
    pub name: String,
    /// Filter description
    pub description: Option<String>,
    /// Filter criteria
    pub criteria: FilterCriteria,
    /// Filter actions
    pub actions: Vec<FilterAction>,
    /// Filter enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCriteria {
    /// Status criteria
    pub status: Option<TaskStatus>,
    /// Priority criteria
    pub priority: Option<Priority>,
    /// Tag criteria
    pub tags: Option<Vec<String>>,
    /// Date criteria
    pub date_range: Option<DateRange>,
    /// Text criteria
    pub text_search: Option<String>,
    /// Custom criteria
    pub custom: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterAction {
    /// Apply tag
    ApplyTag(String),
    /// Set priority
    SetPriority(Priority),
    /// Set status
    SetStatus(TaskStatus),
    /// Move to project
    MoveToProject(ProjectId),
    /// Send notification
    SendNotification(String),
    /// Execute command
    ExecuteCommand(String),
}

impl FilterManager {
    /// Create filter
    pub fn create_filter(&mut self, filter: FilterDefinition) -> Result<(), FilterError> {
        // Validate filter
        self.validate_filter(&filter)?;

        // Store filter
        self.filters.insert(filter.id.clone(), filter);

        Ok(())
    }

    /// Apply filters to task
    pub fn apply_filters(&self, task: &Task) -> Result<Vec<FilterAction>, FilterError> {
        let mut actions = Vec::new();

        for filter in self.filters.values() {
            if !filter.enabled {
                continue;
            }

            if self.matches_criteria(task, &filter.criteria)? {
                actions.extend(filter.actions.clone());
            }
        }

        Ok(actions)
    }

    /// Check if task matches criteria
    fn matches_criteria(&self, task: &Task, criteria: &FilterCriteria) -> Result<bool, FilterError> {
        // Check status
        if let Some(status) = &criteria.status {
            if task.status != *status {
                return Ok(false);
            }
        }

        // Check priority
        if let Some(priority) = &criteria.priority {
            if task.priority != *priority {
                return Ok(false);
            }
        }

        // Check tags
        if let Some(tags) = &criteria.tags {
            for tag in tags {
                if !task.tags.contains(tag) {
                    return Ok(false);
                }
            }
        }

        // Check date range
        if let Some(date_range) = &criteria.date_range {
            if let Some(due_date) = task.due_date {
                if due_date < date_range.start || due_date > date_range.end {
                    return Ok(false);
                }
            }
        }

        // Check text search
        if let Some(text_search) = &criteria.text_search {
            let search_text = format!("{} {}", task.title, task.description.as_deref().unwrap_or(""));
            if !search_text.to_lowercase().contains(&text_search.to_lowercase()) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Validate filter
    fn validate_filter(&self, filter: &FilterDefinition) -> Result<(), FilterError> {
        // Check for required fields
        if filter.name.is_empty() {
            return Err(FilterError::InvalidName);
        }

        // Validate criteria
        self.validate_criteria(&filter.criteria)?;

        // Validate actions
        for action in &filter.actions {
            self.validate_action(action)?;
        }

        Ok(())
    }
}
```

## Monitoring and Metrics

### Task Metrics

```rust
#[derive(Debug, Clone)]
pub struct TaskMetrics {
    /// Total tasks
    pub total_tasks: u64,
    /// Tasks by status
    pub tasks_by_status: HashMap<TaskStatus, u64>,
    /// Tasks by priority
    pub tasks_by_priority: HashMap<Priority, u64>,
    /// Average completion time
    pub avg_completion_time: Duration,
    /// Task creation rate
    pub creation_rate: f64,
    /// Task completion rate
    pub completion_rate: f64,
    /// Popular tags
    pub popular_tags: Vec<TagUsage>,
    /// Project statistics
    pub project_stats: ProjectStatistics,
}

#[derive(Debug, Clone)]
pub struct TagUsage {
    /// Tag name
    pub name: String,
    /// Usage count
    pub count: u64,
    /// Usage percentage
    pub percentage: f64,
}

#[derive(Debug, Clone)]
pub struct ProjectStatistics {
    /// Total projects
    pub total_projects: u64,
    /// Active projects
    pub active_projects: u64,
    /// Completed projects
    pub completed_projects: u64,
    /// Average project size
    pub avg_project_size: f64,
}
```

## Taskwarrior Feature Parity

Edda aims to achieve feature parity with Taskwarrior, a comprehensive command-line task management tool. See [SPEC_TASKWARRIOR_PARITY.md](./SPEC_TASKWARRIOR_PARITY.md) for detailed feature parity requirements.

### Key Taskwarrior Features to Implement

#### Core Features (MVP)

- Basic task operations (add, list, modify, done, delete)
- Task attributes (status, priority, dates, description)
- Basic filtering and search
- Local data storage

#### Advanced Features (Phase 2+)

- Task dependencies and projects
- Tags and annotations
- Recurring tasks and templates
- Time tracking and calendar integration
- Advanced reporting and configuration
- Data management and external integrations

## MVP/Phase 0 Requirements

- Edda must support simple task management (create, list, update, complete, delete)
- Edda must support syncing tasks with GitHub Issues (see Sync Engine spec for details)
- This is a core part of the MVP and required for internal use

> When these requirements are met, Edda will be used internally to track development.

This specification provides a comprehensive task management system that ensures efficient task tracking, organization, and workflow management for AI agent operations.
