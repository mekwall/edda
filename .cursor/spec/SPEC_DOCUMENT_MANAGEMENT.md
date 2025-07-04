# Edda - Document Management Specification

## Overview

This specification defines the document management system for Edda, providing comprehensive document storage, versioning, and organization capabilities for AI agents. The document management system supports multiple content types, metadata management, and integration with task workflows.

## Architecture Context

This component operates within the Edda architecture as defined in [SPEC_ARCHITECTURE.md](./SPEC_ARCHITECTURE.md):

- **Layer**: Core Engine Layer
- **Component**: Document Manager
- **Responsibilities**: Execute business logic for document operations, manage document relationships and metadata, ensure data consistency and integrity, handle document lifecycle management
- **Dependencies**: Data Storage Layer (for document persistence), CLI Interface Layer (for document commands), AI Agent Integration Layer (for AI agent document interactions)

## Architecture Overview

### Document Management Design

The document management system implements the Core Engine Layer responsibilities defined in the master architecture:

- **Document Creator**: Create and validate new documents
- **Content Manager**: Manage document content and processing
- **Metadata Manager**: Handle document metadata and relationships
- **Version Manager**: Manage document versioning and history
- **Index Manager**: Provide document indexing and search capabilities
- **Search Engine**: Enable full-text search and content discovery

## Core Document Components

### Document Engine

```rust
pub struct DocumentEngine {
    /// Document storage
    storage: Arc<dyn DocumentStorage>,
    /// Content manager
    content_manager: Arc<ContentManager>,
    /// Version manager
    version_manager: Arc<VersionManager>,
    /// Metadata manager
    metadata_manager: Arc<MetadataManager>,
    /// Index manager
    index_manager: Arc<IndexManager>,
}

impl DocumentEngine {
    /// Create a new document
    pub fn create_document(&self, document_data: &DocumentData) -> Result<Document, DocumentError> {
        // Validate document data
        self.validate_document_data(document_data)?;

        // Create document
        let document = Document {
            id: DocumentId(Uuid::new_v4().to_string()),
            title: document_data.title.clone(),
            content: document_data.content.clone(),
            doc_type: document_data.doc_type.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            task_ids: document_data.task_ids.clone(),
            tags: document_data.tags.clone(),
            version: Version::new(1, 0, 0),
            metadata: document_data.metadata.clone(),
            agent_context: document_data.agent_context.clone(),
        };

        // Store document
        self.storage.store_document(&document)?;

        // Process content
        self.content_manager.process_content(&document)?;

        // Create initial version
        self.version_manager.create_version(&document)?;

        // Index document
        self.index_manager.index_document(&document)?;

        Ok(document)
    }

    /// Update an existing document
    pub fn update_document(&self, document_id: &DocumentId, updates: &DocumentUpdates) -> Result<Document, DocumentError> {
        // Get existing document
        let mut document = self.storage.get_document(document_id)?
            .ok_or(DocumentError::DocumentNotFound)?;

        // Create new version if content changed
        let content_changed = if let Some(new_content) = &updates.content {
            &document.content != new_content
        } else {
            false
        };

        // Apply updates
        if let Some(title) = &updates.title {
            document.title = title.clone();
        }
        if let Some(content) = &updates.content {
            document.content = content.clone();
        }
        if let Some(doc_type) = &updates.doc_type {
            document.doc_type = doc_type.clone();
        }
        if let Some(tags) = &updates.tags {
            document.tags = tags.clone();
        }
        if let Some(metadata) = &updates.metadata {
            document.metadata.extend(metadata.clone());
        }

        // Update timestamps
        document.updated_at = Utc::now();

        // Create new version if content changed
        if content_changed {
            document.version = self.version_manager.increment_version(&document.version)?;
            self.version_manager.create_version(&document)?;
        }

        // Store updated document
        self.storage.update_document(&document)?;

        // Re-index document
        self.index_manager.reindex_document(&document)?;

        Ok(document)
    }

    /// Delete a document
    pub fn delete_document(&self, document_id: &DocumentId) -> Result<(), DocumentError> {
        // Get document
        let document = self.storage.get_document(document_id)?
            .ok_or(DocumentError::DocumentNotFound)?;

        // Remove from tasks
        for task_id in &document.task_ids {
            self.remove_document_from_task(task_id, document_id)?;
        }

        // Delete document
        self.storage.delete_document(document_id)?;

        // Remove from index
        self.index_manager.remove_document(document_id)?;

        // Delete versions
        self.version_manager.delete_versions(document_id)?;

        Ok(())
    }

    /// Get document by ID
    pub fn get_document(&self, document_id: &DocumentId) -> Result<Option<Document>, DocumentError> {
        self.storage.get_document(document_id)
    }

    /// Query documents
    pub fn query_documents(&self, query: &DocumentQuery) -> Result<Vec<Document>, DocumentError> {
        self.storage.query_documents(query)
    }

    /// Search documents
    pub fn search_documents(&self, search_query: &SearchQuery) -> Result<Vec<SearchResult>, DocumentError> {
        self.index_manager.search_documents(search_query)
    }

    /// Validate document data
    fn validate_document_data(&self, data: &DocumentData) -> Result<(), DocumentError> {
        // Check required fields
        if data.title.is_empty() {
            return Err(DocumentError::InvalidTitle);
        }

        // Validate content
        self.content_manager.validate_content(&data.content)?;

        // Validate document type
        self.validate_document_type(&data.doc_type)?;

        Ok(())
    }
}
```

## Document Data Models

### Document Data

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentData {
    /// Document title
    pub title: String,
    /// Document content
    pub content: DocumentContent,
    /// Document type
    pub doc_type: DocumentType,
    /// Associated task IDs
    pub task_ids: Vec<TaskId>,
    /// Document tags
    pub tags: Vec<String>,
    /// Custom metadata
    pub metadata: HashMap<String, Value>,
    /// AI agent context
    pub agent_context: Option<AgentContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentUpdates {
    /// Updated title
    pub title: Option<String>,
    /// Updated content
    pub content: Option<DocumentContent>,
    /// Updated document type
    pub doc_type: Option<DocumentType>,
    /// Updated tags
    pub tags: Option<Vec<String>>,
    /// Updated metadata
    pub metadata: Option<HashMap<String, Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentQuery {
    /// Document type filter
    pub doc_type: Option<DocumentType>,
    /// Tag filter
    pub tags: Option<Vec<String>>,
    /// Task ID filter
    pub task_id: Option<TaskId>,
    /// Date range filter
    pub date_range: Option<DateRange>,
    /// Text search
    pub search: Option<String>,
    /// Agent context filter
    pub agent_context: Option<AgentContext>,
    /// Sort criteria
    pub sort: Vec<SortCriteria>,
    /// Pagination
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    /// Search text
    pub query: String,
    /// Search fields
    pub fields: Vec<String>,
    /// Search options
    pub options: SearchOptions,
    /// Result limit
    pub limit: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchOptions {
    /// Case sensitive search
    pub case_sensitive: bool,
    /// Fuzzy search
    pub fuzzy: bool,
    /// Search in metadata
    pub include_metadata: bool,
    /// Search in tags
    pub include_tags: bool,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    /// Document ID
    pub document_id: DocumentId,
    /// Relevance score
    pub score: f64,
    /// Highlighted matches
    pub highlights: Vec<Highlight>,
    /// Snippet
    pub snippet: String,
}

#[derive(Debug, Clone)]
pub struct Highlight {
    /// Field name
    pub field: String,
    /// Start position
    pub start: usize,
    /// End position
    pub end: usize,
    /// Highlighted text
    pub text: String,
}
```

## Content Management

### Content Manager

```rust
pub struct ContentManager {
    /// Content processors
    processors: HashMap<DocumentType, Box<dyn ContentProcessor>>,
    /// Content validators
    validators: HashMap<DocumentType, Box<dyn ContentValidator>>,
    /// Content transformers
    transformers: HashMap<DocumentType, Box<dyn ContentTransformer>>,
}

impl ContentManager {
    /// Process document content
    pub fn process_content(&self, document: &Document) -> Result<(), ContentError> {
        let processor = self.processors.get(&document.doc_type)
            .ok_or(ContentError::NoProcessor)?;

        processor.process(&document.content)?;
        Ok(())
    }

    /// Validate content
    pub fn validate_content(&self, content: &DocumentContent) -> Result<(), ContentError> {
        match content {
            DocumentContent::Text(text) => {
                self.validate_text_content(text)
            }
            DocumentContent::Markdown(markdown) => {
                self.validate_markdown_content(markdown)
            }
            DocumentContent::Json(json) => {
                self.validate_json_content(json)
            }
            DocumentContent::Yaml(yaml) => {
                self.validate_yaml_content(yaml)
            }
            DocumentContent::Binary { data, mime_type, .. } => {
                self.validate_binary_content(data, mime_type)
            }
            DocumentContent::FileReference { path, .. } => {
                self.validate_file_reference(path)
            }
        }
    }

    /// Transform content
    pub fn transform_content(&self, content: &DocumentContent, target_type: DocumentType) -> Result<DocumentContent, ContentError> {
        let transformer = self.transformers.get(&target_type)
            .ok_or(ContentError::NoTransformer)?;

        transformer.transform(content)
    }

    /// Extract text from content
    pub fn extract_text(&self, content: &DocumentContent) -> Result<String, ContentError> {
        match content {
            DocumentContent::Text(text) => Ok(text.clone()),
            DocumentContent::Markdown(markdown) => {
                self.extract_text_from_markdown(markdown)
            }
            DocumentContent::Json(json) => {
                self.extract_text_from_json(json)
            }
            DocumentContent::Yaml(yaml) => {
                self.extract_text_from_yaml(yaml)
            }
            DocumentContent::Binary { data, mime_type, .. } => {
                self.extract_text_from_binary(data, mime_type)
            }
            DocumentContent::FileReference { path, .. } => {
                self.extract_text_from_file(path)
            }
        }
    }

    /// Validate text content
    fn validate_text_content(&self, text: &str) -> Result<(), ContentError> {
        // Check for empty content
        if text.trim().is_empty() {
            return Err(ContentError::EmptyContent);
        }

        // Check for maximum size
        if text.len() > MAX_TEXT_SIZE {
            return Err(ContentError::ContentTooLarge);
        }

        Ok(())
    }

    /// Validate markdown content
    fn validate_markdown_content(&self, markdown: &str) -> Result<(), ContentError> {
        // Validate markdown syntax
        self.validate_markdown_syntax(markdown)?;

        // Check for maximum size
        if markdown.len() > MAX_MARKDOWN_SIZE {
            return Err(ContentError::ContentTooLarge);
        }

        Ok(())
    }

    /// Validate JSON content
    fn validate_json_content(&self, json: &Value) -> Result<(), ContentError> {
        // Validate JSON structure
        self.validate_json_structure(json)?;

        // Check for maximum depth
        if self.get_json_depth(json) > MAX_JSON_DEPTH {
            return Err(ContentError::ContentTooComplex);
        }

        Ok(())
    }
}
```

## Version Management

### Version Manager

```rust
pub struct VersionManager {
    /// Version storage
    storage: Arc<dyn VersionStorage>,
    /// Version policy
    policy: VersionPolicy,
}

#[derive(Debug, Clone)]
pub struct VersionPolicy {
    /// Maximum versions to keep
    pub max_versions: usize,
    /// Auto-cleanup enabled
    pub auto_cleanup: bool,
    /// Version retention period
    pub retention_period: Duration,
    /// Version naming strategy
    pub naming_strategy: VersionNamingStrategy,
}

#[derive(Debug, Clone)]
pub enum VersionNamingStrategy {
    /// Semantic versioning
    Semantic,
    /// Timestamp-based
    Timestamp,
    /// Incremental
    Incremental,
    /// Custom
    Custom(String),
}

impl VersionManager {
    /// Create version
    pub fn create_version(&self, document: &Document) -> Result<Version, VersionError> {
        let version = Version {
            id: VersionId(Uuid::new_v4().to_string()),
            document_id: document.id.clone(),
            version: document.version.clone(),
            content: document.content.clone(),
            metadata: document.metadata.clone(),
            created_at: Utc::now(),
            created_by: document.agent_context.as_ref().map(|ctx| ctx.agent_id.clone()),
            message: None,
        };

        self.storage.store_version(&version)?;

        // Apply version policy
        self.apply_version_policy(&document.id)?;

        Ok(version)
    }

    /// Get version history
    pub fn get_version_history(&self, document_id: &DocumentId) -> Result<Vec<Version>, VersionError> {
        let versions = self.storage.get_versions(document_id)?;

        // Sort by version number
        let mut sorted_versions = versions;
        sorted_versions.sort_by(|a, b| a.version.cmp(&b.version));

        Ok(sorted_versions)
    }

    /// Get specific version
    pub fn get_version(&self, document_id: &DocumentId, version: &Version) -> Result<Option<Version>, VersionError> {
        self.storage.get_version(document_id, version)
    }

    /// Revert to version
    pub fn revert_to_version(&self, document_id: &DocumentId, version: &Version) -> Result<Document, VersionError> {
        let version_data = self.storage.get_version(document_id, version)?
            .ok_or(VersionError::VersionNotFound)?;

        // Get current document
        let mut document = self.storage.get_document(document_id)?
            .ok_or(VersionError::DocumentNotFound)?;

        // Revert content and metadata
        document.content = version_data.content;
        document.metadata = version_data.metadata;
        document.updated_at = Utc::now();

        // Create new version for revert
        let new_version = self.increment_version(&document.version)?;
        document.version = new_version;

        // Store updated document
        self.storage.update_document(&document)?;

        // Create version record
        self.create_version(&document)?;

        Ok(document)
    }

    /// Compare versions
    pub fn compare_versions(&self, document_id: &DocumentId, version1: &Version, version2: &Version) -> Result<VersionDiff, VersionError> {
        let v1 = self.storage.get_version(document_id, version1)?
            .ok_or(VersionError::VersionNotFound)?;
        let v2 = self.storage.get_version(document_id, version2)?
            .ok_or(VersionError::VersionNotFound)?;

        let diff = VersionDiff {
            document_id: document_id.clone(),
            old_version: v1.version.clone(),
            new_version: v2.version.clone(),
            content_changes: self.diff_content(&v1.content, &v2.content)?,
            metadata_changes: self.diff_metadata(&v1.metadata, &v2.metadata)?,
            timestamp_diff: v2.created_at - v1.created_at,
        };

        Ok(diff)
    }

    /// Increment version
    fn increment_version(&self, version: &Version) -> Result<Version, VersionError> {
        match &version {
            Version::Semantic(major, minor, patch) => {
                Ok(Version::Semantic(*major, *minor, patch + 1))
            }
            Version::Timestamp(timestamp) => {
                Ok(Version::Timestamp(Utc::now()))
            }
            Version::Incremental(number) => {
                Ok(Version::Incremental(number + 1))
            }
        }
    }

    /// Apply version policy
    fn apply_version_policy(&self, document_id: &DocumentId) -> Result<(), VersionError> {
        if !self.policy.auto_cleanup {
            return Ok(());
        }

        let versions = self.storage.get_versions(document_id)?;

        if versions.len() > self.policy.max_versions {
            // Sort by creation time
            let mut sorted_versions = versions;
            sorted_versions.sort_by(|a, b| a.created_at.cmp(&b.created_at));

            // Remove oldest versions
            let to_remove = sorted_versions.len() - self.policy.max_versions;
            for version in sorted_versions.iter().take(to_remove) {
                self.storage.delete_version(document_id, &version.version)?;
            }
        }

        Ok(())
    }
}
```

## Metadata Management

### Metadata Manager

```rust
pub struct MetadataManager {
    /// Metadata storage
    storage: Arc<dyn MetadataStorage>,
    /// Metadata schemas
    schemas: HashMap<String, MetadataSchema>,
    /// Metadata validators
    validators: HashMap<String, Box<dyn MetadataValidator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataSchema {
    /// Schema ID
    pub id: String,
    /// Schema name
    pub name: String,
    /// Schema description
    pub description: Option<String>,
    /// Schema fields
    pub fields: Vec<MetadataField>,
    /// Schema validation rules
    pub validation_rules: Vec<ValidationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataField {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: MetadataFieldType,
    /// Field description
    pub description: Option<String>,
    /// Field required
    pub required: bool,
    /// Field default value
    pub default_value: Option<Value>,
    /// Field validation rules
    pub validation_rules: Vec<ValidationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetadataFieldType {
    /// String field
    String,
    /// Number field
    Number,
    /// Boolean field
    Boolean,
    /// Date field
    Date,
    /// Array field
    Array(Box<MetadataFieldType>),
    /// Object field
    Object(Vec<MetadataField>),
}

impl MetadataManager {
    /// Register metadata schema
    pub fn register_schema(&mut self, schema: MetadataSchema) -> Result<(), MetadataError> {
        // Validate schema
        self.validate_schema(&schema)?;

        // Register schema
        self.schemas.insert(schema.id.clone(), schema);

        Ok(())
    }

    /// Validate metadata
    pub fn validate_metadata(&self, metadata: &HashMap<String, Value>, schema_id: &str) -> Result<(), MetadataError> {
        let schema = self.schemas.get(schema_id)
            .ok_or(MetadataError::SchemaNotFound)?;

        // Validate required fields
        for field in &schema.fields {
            if field.required {
                if !metadata.contains_key(&field.name) {
                    return Err(MetadataError::MissingRequiredField(field.name.clone()));
                }
            }
        }

        // Validate field types
        for (key, value) in metadata {
            if let Some(field) = schema.fields.iter().find(|f| &f.name == key) {
                self.validate_field_value(value, field)?;
            }
        }

        // Apply schema validation rules
        for rule in &schema.validation_rules {
            rule.validate(metadata)?;
        }

        Ok(())
    }

    /// Extract metadata from content
    pub fn extract_metadata(&self, content: &DocumentContent) -> Result<HashMap<String, Value>, MetadataError> {
        let mut metadata = HashMap::new();

        match content {
            DocumentContent::Text(text) => {
                self.extract_text_metadata(text, &mut metadata)?;
            }
            DocumentContent::Markdown(markdown) => {
                self.extract_markdown_metadata(markdown, &mut metadata)?;
            }
            DocumentContent::Json(json) => {
                self.extract_json_metadata(json, &mut metadata)?;
            }
            DocumentContent::Yaml(yaml) => {
                self.extract_yaml_metadata(yaml, &mut metadata)?;
            }
            DocumentContent::Binary { data, mime_type, filename } => {
                self.extract_binary_metadata(data, mime_type, filename.as_deref(), &mut metadata)?;
            }
            DocumentContent::FileReference { path, .. } => {
                self.extract_file_metadata(path, &mut metadata)?;
            }
        }

        Ok(metadata)
    }

    /// Validate field value
    fn validate_field_value(&self, value: &Value, field: &MetadataField) -> Result<(), MetadataError> {
        match &field.field_type {
            MetadataFieldType::String => {
                if !value.is_string() {
                    return Err(MetadataError::InvalidFieldType(field.name.clone()));
                }
            }
            MetadataFieldType::Number => {
                if !value.is_number() {
                    return Err(MetadataError::InvalidFieldType(field.name.clone()));
                }
            }
            MetadataFieldType::Boolean => {
                if !value.is_boolean() {
                    return Err(MetadataError::InvalidFieldType(field.name.clone()));
                }
            }
            MetadataFieldType::Date => {
                if !value.is_string() {
                    return Err(MetadataError::InvalidFieldType(field.name.clone()));
                }
                // Validate date format
                if let Some(date_str) = value.as_str() {
                    if DateTime::parse_from_rfc3339(date_str).is_err() {
                        return Err(MetadataError::InvalidFieldFormat(field.name.clone()));
                    }
                }
            }
            MetadataFieldType::Array(element_type) => {
                if !value.is_array() {
                    return Err(MetadataError::InvalidFieldType(field.name.clone()));
                }
                // Validate array elements
                if let Some(array) = value.as_array() {
                    for element in array {
                        self.validate_field_value(element, &MetadataField {
                            name: format!("{}[]", field.name),
                            field_type: *element_type.clone(),
                            description: None,
                            required: false,
                            default_value: None,
                            validation_rules: vec![],
                        })?;
                    }
                }
            }
            MetadataFieldType::Object(fields) => {
                if !value.is_object() {
                    return Err(MetadataError::InvalidFieldType(field.name.clone()));
                }
                // Validate object fields
                if let Some(obj) = value.as_object() {
                    for field_def in fields {
                        if let Some(field_value) = obj.get(&field_def.name) {
                            self.validate_field_value(field_value, field_def)?;
                        }
                    }
                }
            }
        }

        // Apply field validation rules
        for rule in &field.validation_rules {
            rule.validate_field(value, field)?;
        }

        Ok(())
    }
}
```

## Index Management

### Index Manager

```rust
pub struct IndexManager {
    /// Index storage
    storage: Arc<dyn IndexStorage>,
    /// Search engine
    search_engine: Arc<SearchEngine>,
    /// Index configuration
    config: IndexConfig,
}

#[derive(Debug, Clone)]
pub struct IndexConfig {
    /// Index type
    pub index_type: IndexType,
    /// Index fields
    pub fields: Vec<IndexField>,
    /// Index options
    pub options: IndexOptions,
}

#[derive(Debug, Clone)]
pub enum IndexType {
    /// Full-text search index
    FullText,
    /// Metadata index
    Metadata,
    /// Combined index
    Combined,
}

#[derive(Debug, Clone)]
pub struct IndexField {
    /// Field name
    pub name: String,
    /// Field type
    pub field_type: IndexFieldType,
    /// Field options
    pub options: IndexFieldOptions,
}

#[derive(Debug, Clone)]
pub enum IndexFieldType {
    /// Text field
    Text,
    /// Keyword field
    Keyword,
    /// Number field
    Number,
    /// Date field
    Date,
    /// Boolean field
    Boolean,
}

impl IndexManager {
    /// Index document
    pub fn index_document(&self, document: &Document) -> Result<(), IndexError> {
        // Extract indexable content
        let indexable_content = self.extract_indexable_content(document)?;

        // Create index entry
        let index_entry = IndexEntry {
            document_id: document.id.clone(),
            title: document.title.clone(),
            content: indexable_content,
            metadata: document.metadata.clone(),
            tags: document.tags.clone(),
            doc_type: document.doc_type.clone(),
            created_at: document.created_at,
            updated_at: document.updated_at,
        };

        // Store index entry
        self.storage.store_index_entry(&index_entry)?;

        // Update search engine
        self.search_engine.index_document(&index_entry)?;

        Ok(())
    }

    /// Re-index document
    pub fn reindex_document(&self, document: &Document) -> Result<(), IndexError> {
        // Remove old index entry
        self.remove_document(&document.id)?;

        // Create new index entry
        self.index_document(document)?;

        Ok(())
    }

    /// Remove document from index
    pub fn remove_document(&self, document_id: &DocumentId) -> Result<(), IndexError> {
        self.storage.remove_index_entry(document_id)?;
        self.search_engine.remove_document(document_id)?;
        Ok(())
    }

    /// Search documents
    pub fn search_documents(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, IndexError> {
        self.search_engine.search(query)
    }

    /// Extract indexable content
    fn extract_indexable_content(&self, document: &Document) -> Result<String, IndexError> {
        let mut content = String::new();

        // Add title
        content.push_str(&document.title);
        content.push(' ');

        // Add content text
        let text_content = self.content_manager.extract_text(&document.content)?;
        content.push_str(&text_content);
        content.push(' ');

        // Add tags
        for tag in &document.tags {
            content.push_str(tag);
            content.push(' ');
        }

        // Add metadata text
        for (key, value) in &document.metadata {
            if let Some(value_str) = value.as_str() {
                content.push_str(value_str);
                content.push(' ');
            }
        }

        Ok(content)
    }
}
```

## Monitoring and Metrics

### Document Metrics

```rust
#[derive(Debug, Clone)]
pub struct DocumentMetrics {
    /// Total documents
    pub total_documents: u64,
    /// Documents by type
    pub documents_by_type: HashMap<DocumentType, u64>,
    /// Documents by size
    pub documents_by_size: HashMap<SizeRange, u64>,
    /// Average document size
    pub avg_document_size: u64,
    /// Document creation rate
    pub creation_rate: f64,
    /// Document update rate
    pub update_rate: f64,
    /// Popular tags
    pub popular_tags: Vec<TagUsage>,
    /// Version statistics
    pub version_stats: VersionStatistics,
    /// Index statistics
    pub index_stats: IndexStatistics,
}

#[derive(Debug, Clone)]
pub struct SizeRange {
    /// Minimum size in bytes
    pub min_size: u64,
    /// Maximum size in bytes
    pub max_size: u64,
    /// Range name
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct VersionStatistics {
    /// Total versions
    pub total_versions: u64,
    /// Average versions per document
    pub avg_versions_per_document: f64,
    /// Version creation rate
    pub creation_rate: f64,
    /// Version retention rate
    pub retention_rate: f64,
}

#[derive(Debug, Clone)]
pub struct IndexStatistics {
    /// Indexed documents
    pub indexed_documents: u64,
    /// Index size
    pub index_size: u64,
    /// Search queries
    pub search_queries: u64,
    /// Average search time
    pub avg_search_time: Duration,
    /// Cache hit rate
    pub cache_hit_rate: f64,
}
```

This specification provides a comprehensive document management system that ensures efficient document storage, versioning, and organization for AI agent operations.
