# Edda - Query Engine Specification

## Overview

This specification defines the query engine architecture for Edda, providing powerful and efficient querying capabilities for AI agent task and document management. The query engine supports complex queries, optimization, and real-time processing.

## Architecture Context

This component operates within the Edda architecture as defined in [SPEC_ARCHITECTURE.md](./SPEC_ARCHITECTURE.md):

- **Layer**: Core Engine Layer
- **Component**: Query Engine
- **Responsibilities**: Process SQL-like queries across data types, provide query and search capabilities, handle caching and performance optimization
- **Dependencies**: Data Storage Layer (for data access), CLI Interface Layer (for query input/output), AI Agent Integration Layer (for AI agent queries)

## Architecture Overview

### Query Engine Design

The query engine implements the Core Engine Layer responsibilities defined in the master architecture:

- **Query Parser**: Parse and validate SQL-like queries
- **Query Planner**: Plan optimal query execution strategies
- **Query Optimizer**: Optimize queries for performance
- **Query Executor**: Execute queries against storage backends
- **Result Formatter**: Format query results for different output formats
- **Cache Manager**: Cache frequently accessed query results

## Query Language

### Query Syntax

The query engine supports a SQL-like syntax optimized for task and document management:

```sql
-- Basic task queries
SELECT * FROM tasks WHERE status = 'pending' AND priority >= 3;

-- Complex queries with joins
SELECT t.title, d.title as document_title
FROM tasks t
JOIN task_documents td ON t.id = td.task_id
JOIN documents d ON td.document_id = d.id
WHERE t.status = 'in_progress';

-- Aggregation queries
SELECT status, COUNT(*) as count, AVG(priority) as avg_priority
FROM tasks
GROUP BY status
HAVING count > 5;

-- Full-text search
SELECT * FROM documents
WHERE MATCH(content) AGAINST('AI agent workflow');

-- Time-based queries
SELECT * FROM tasks
WHERE created_at >= '2024-01-01'
AND due_date <= '2024-12-31';
```

### Query Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryType {
    /// Simple selection query
    Select(SelectQuery),
    /// Insert operation
    Insert(InsertQuery),
    /// Update operation
    Update(UpdateQuery),
    /// Delete operation
    Delete(DeleteQuery),
    /// Full-text search
    Search(SearchQuery),
    /// Aggregation query
    Aggregate(AggregateQuery),
    /// Custom query
    Custom(CustomQuery),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectQuery {
    /// Target table
    pub table: String,
    /// Selected columns
    pub columns: Vec<Column>,
    /// Where conditions
    pub where_clause: Option<Expression>,
    /// Join clauses
    pub joins: Vec<JoinClause>,
    /// Group by clause
    pub group_by: Vec<Column>,
    /// Having clause
    pub having: Option<Expression>,
    /// Order by clause
    pub order_by: Vec<OrderByClause>,
    /// Limit clause
    pub limit: Option<u64>,
    /// Offset clause
    pub offset: Option<u64>,
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
```

## Query Processing Pipeline

### Query Parser

```rust
pub struct QueryParser {
    /// Parser configuration
    config: ParserConfig,
    /// Error handler
    error_handler: Box<dyn ErrorHandler>,
}

impl QueryParser {
    /// Parse SQL query string
    pub fn parse(&self, sql: &str) -> Result<Query, ParseError> {
        // Tokenize the SQL
        let tokens = self.tokenize(sql)?;

        // Parse tokens into AST
        let ast = self.parse_tokens(&tokens)?;

        // Validate AST
        self.validate_ast(&ast)?;

        // Convert to query object
        Ok(self.ast_to_query(ast)?)
    }

    /// Tokenize SQL string
    fn tokenize(&self, sql: &str) -> Result<Vec<Token>, ParseError> {
        let mut tokens = Vec::new();
        let mut current = 0;

        while current < sql.len() {
            let (token, next) = self.next_token(&sql[current..])?;
            tokens.push(token);
            current += next;
        }

        Ok(tokens)
    }

    /// Parse tokens into AST
    fn parse_tokens(&self, tokens: &[Token]) -> Result<AstNode, ParseError> {
        // Recursive descent parsing
        self.parse_statement(tokens, 0)
    }
}
```

### Query Planner

```rust
pub struct QueryPlanner {
    /// Planning configuration
    config: PlannerConfig,
    /// Statistics collector
    stats_collector: Box<dyn StatisticsCollector>,
}

impl QueryPlanner {
    /// Plan query execution
    pub fn plan(&self, query: &Query) -> Result<ExecutionPlan, PlanningError> {
        // Analyze query structure
        let analysis = self.analyze_query(query)?;

        // Generate candidate plans
        let candidates = self.generate_candidates(query, &analysis)?;

        // Cost-based optimization
        let best_plan = self.optimize_plans(candidates)?;

        // Finalize execution plan
        Ok(self.finalize_plan(best_plan)?)
    }

    /// Analyze query for planning
    fn analyze_query(&self, query: &Query) -> Result<QueryAnalysis, PlanningError> {
        let analysis = QueryAnalysis {
            tables: self.extract_tables(query)?,
            indexes: self.identify_indexes(query)?,
            filters: self.extract_filters(query)?,
            joins: self.extract_joins(query)?,
            aggregations: self.extract_aggregations(query)?,
            estimated_rows: self.estimate_row_count(query)?,
        };

        Ok(analysis)
    }

    /// Generate candidate execution plans
    fn generate_candidates(&self, query: &Query, analysis: &QueryAnalysis) -> Result<Vec<ExecutionPlan>, PlanningError> {
        let mut candidates = Vec::new();

        // Generate table scan plans
        candidates.extend(self.generate_scan_plans(query, analysis)?);

        // Generate index-based plans
        candidates.extend(self.generate_index_plans(query, analysis)?);

        // Generate join plans
        candidates.extend(self.generate_join_plans(query, analysis)?);

        // Generate aggregation plans
        candidates.extend(self.generate_aggregation_plans(query, analysis)?);

        Ok(candidates)
    }
}
```

### Query Optimizer

```rust
pub struct QueryOptimizer {
    /// Optimization configuration
    config: OptimizerConfig,
    /// Cost model
    cost_model: Box<dyn CostModel>,
}

impl QueryOptimizer {
    /// Optimize execution plan
    pub fn optimize(&self, plan: ExecutionPlan) -> Result<ExecutionPlan, OptimizationError> {
        // Apply rewrite rules
        let rewritten = self.apply_rewrite_rules(plan)?;

        // Cost-based optimization
        let optimized = self.cost_based_optimization(rewritten)?;

        // Physical optimization
        let physical = self.physical_optimization(optimized)?;

        Ok(physical)
    }

    /// Apply rewrite rules
    fn apply_rewrite_rules(&self, plan: ExecutionPlan) -> Result<ExecutionPlan, OptimizationError> {
        let mut current = plan;

        // Predicate pushdown
        current = self.predicate_pushdown(current)?;

        // Join reordering
        current = self.join_reordering(current)?;

        // Subquery optimization
        current = self.subquery_optimization(current)?;

        // Expression simplification
        current = self.expression_simplification(current)?;

        Ok(current)
    }

    /// Cost-based optimization
    fn cost_based_optimization(&self, plan: ExecutionPlan) -> Result<ExecutionPlan, OptimizationError> {
        // Calculate costs for different alternatives
        let alternatives = self.generate_alternatives(&plan)?;

        // Select lowest cost plan
        let best_plan = alternatives.into_iter()
            .min_by_key(|plan| self.calculate_cost(plan))
            .ok_or(OptimizationError::NoValidPlan)?;

        Ok(best_plan)
    }
}
```

## Execution Engine

### Query Executor

```rust
pub struct QueryExecutor {
    /// Execution configuration
    config: ExecutorConfig,
    /// Storage engine
    storage: Arc<dyn StorageEngine>,
    /// Cache manager
    cache: Arc<CacheManager>,
}

impl QueryExecutor {
    /// Execute query
    pub fn execute(&self, plan: &ExecutionPlan) -> Result<QueryResult, ExecutionError> {
        // Check cache first
        if let Some(cached_result) = self.check_cache(plan)? {
            return Ok(cached_result);
        }

        // Execute the plan
        let result = self.execute_plan(plan)?;

        // Cache the result
        self.cache_result(plan, &result)?;

        Ok(result)
    }

    /// Execute execution plan
    fn execute_plan(&self, plan: &ExecutionPlan) -> Result<QueryResult, ExecutionError> {
        match &plan.root {
            ExecutionNode::TableScan(scan) => {
                self.execute_table_scan(scan)
            }
            ExecutionNode::IndexScan(scan) => {
                self.execute_index_scan(scan)
            }
            ExecutionNode::Filter(filter) => {
                self.execute_filter(filter)
            }
            ExecutionNode::Join(join) => {
                self.execute_join(join)
            }
            ExecutionNode::Aggregate(agg) => {
                self.execute_aggregate(agg)
            }
            ExecutionNode::Sort(sort) => {
                self.execute_sort(sort)
            }
            ExecutionNode::Limit(limit) => {
                self.execute_limit(limit)
            }
        }
    }

    /// Execute table scan
    fn execute_table_scan(&self, scan: &TableScanNode) -> Result<QueryResult, ExecutionError> {
        let mut rows = Vec::new();

        // Iterate through table
        let mut iterator = self.storage.scan_table(&scan.table)?;

        while let Some(row) = iterator.next()? {
            // Apply projection
            let projected = self.project_row(row, &scan.projection)?;
            rows.push(projected);
        }

        Ok(QueryResult {
            columns: scan.projection.clone(),
            rows,
            row_count: rows.len(),
        })
    }

    /// Execute index scan
    fn execute_index_scan(&self, scan: &IndexScanNode) -> Result<QueryResult, ExecutionError> {
        let mut rows = Vec::new();

        // Use index for efficient lookup
        let index_iterator = self.storage.scan_index(&scan.index_name, &scan.range)?;

        while let Some(row_id) = index_iterator.next()? {
            // Fetch full row
            let row = self.storage.get_row(&scan.table, row_id)?;

            // Apply projection
            let projected = self.project_row(row, &scan.projection)?;
            rows.push(projected);
        }

        Ok(QueryResult {
            columns: scan.projection.clone(),
            rows,
            row_count: rows.len(),
        })
    }
}
```

## Index Management

### Index Manager

```rust
pub struct IndexManager {
    /// Index configurations
    indexes: HashMap<String, IndexConfig>,
    /// Storage engine
    storage: Arc<dyn StorageEngine>,
    /// Statistics collector
    stats_collector: Box<dyn StatisticsCollector>,
}

impl IndexManager {
    /// Create index
    pub fn create_index(&self, config: &IndexConfig) -> Result<(), IndexError> {
        // Validate index configuration
        self.validate_index_config(config)?;

        // Build index
        self.build_index(config)?;

        // Update index registry
        self.register_index(config)?;

        Ok(())
    }

    /// Build index
    fn build_index(&self, config: &IndexConfig) -> Result<(), IndexError> {
        match config.index_type {
            IndexType::BTree => {
                self.build_btree_index(config)
            }
            IndexType::Hash => {
                self.build_hash_index(config)
            }
            IndexType::FullText => {
                self.build_fulltext_index(config)
            }
            IndexType::Spatial => {
                self.build_spatial_index(config)
            }
        }
    }

    /// Build B-tree index
    fn build_btree_index(&self, config: &IndexConfig) -> Result<(), IndexError> {
        let mut index_data = BTreeMap::new();

        // Scan table and extract index values
        let mut iterator = self.storage.scan_table(&config.table)?;

        while let Some(row) = iterator.next()? {
            let key = self.extract_index_key(row, &config.columns)?;
            let row_id = self.extract_row_id(row)?;

            index_data.insert(key, row_id);
        }

        // Store index
        self.storage.store_index(&config.name, &index_data)?;

        Ok(())
    }
}
```

## Full-Text Search

### Search Engine

```rust
pub struct SearchEngine {
    /// Search configuration
    config: SearchConfig,
    /// Inverted index
    inverted_index: Arc<InvertedIndex>,
    /// Query parser
    query_parser: Box<dyn SearchQueryParser>,
}

impl SearchEngine {
    /// Search documents
    pub fn search(&self, query: &SearchQuery) -> Result<SearchResult, SearchError> {
        // Parse search query
        let parsed_query = self.query_parser.parse(&query.query)?;

        // Execute search
        let results = self.execute_search(parsed_query, query)?;

        // Rank results
        let ranked_results = self.rank_results(results, query)?;

        // Format results
        Ok(SearchResult {
            query: query.query.clone(),
            results: ranked_results,
            total_hits: ranked_results.len(),
            search_time: Duration::from_millis(0), // Calculate actual time
        })
    }

    /// Execute search
    fn execute_search(&self, query: ParsedQuery, search_query: &SearchQuery) -> Result<Vec<SearchHit>, SearchError> {
        let mut hits = Vec::new();

        match query {
            ParsedQuery::Term(term) => {
                hits.extend(self.search_term(&term, search_query)?);
            }
            ParsedQuery::Phrase(phrase) => {
                hits.extend(self.search_phrase(&phrase, search_query)?);
            }
            ParsedQuery::Boolean(boolean) => {
                hits.extend(self.search_boolean(&boolean, search_query)?);
            }
            ParsedQuery::Fuzzy(fuzzy) => {
                hits.extend(self.search_fuzzy(&fuzzy, search_query)?);
            }
        }

        Ok(hits)
    }

    /// Search for term
    fn search_term(&self, term: &str, search_query: &SearchQuery) -> Result<Vec<SearchHit>, SearchError> {
        let mut hits = Vec::new();

        // Look up term in inverted index
        if let Some(posting_list) = self.inverted_index.get_posting_list(term)? {
            for posting in posting_list {
                let document = self.storage.get_document(&posting.document_id)?;

                if let Some(document) = document {
                    // Check if document matches search fields
                    if self.matches_search_fields(&document, &search_query.fields)? {
                        let hit = SearchHit {
                            document_id: posting.document_id.clone(),
                            score: posting.tf_idf,
                            highlights: self.generate_highlights(&document, term)?,
                        };
                        hits.push(hit);
                    }
                }
            }
        }

        Ok(hits)
    }
}
```

## Query Caching

### Cache Manager

```rust
pub struct QueryCacheManager {
    /// Cache configuration
    config: CacheConfig,
    /// Cache storage
    cache: Arc<dyn CacheStorage>,
    /// Query hasher
    query_hasher: Box<dyn QueryHasher>,
}

impl QueryCacheManager {
    /// Get cached result
    pub fn get(&self, query: &Query) -> Result<Option<QueryResult>, CacheError> {
        let key = self.query_hasher.hash(query)?;

        if let Some(cached) = self.cache.get(&key)? {
            // Check if cache is still valid
            if self.is_cache_valid(&cached)? {
                return Ok(Some(cached.result));
            } else {
                // Remove expired cache entry
                self.cache.remove(&key)?;
            }
        }

        Ok(None)
    }

    /// Cache query result
    pub fn put(&self, query: &Query, result: &QueryResult) -> Result<(), CacheError> {
        let key = self.query_hasher.hash(query)?;

        let cache_entry = CacheEntry {
            result: result.clone(),
            created_at: Instant::now(),
            expires_at: Some(Instant::now() + self.config.ttl),
        };

        self.cache.put(&key, &cache_entry)?;

        Ok(())
    }

    /// Check if cache entry is valid
    fn is_cache_valid(&self, entry: &CacheEntry) -> Result<bool, CacheError> {
        if let Some(expires_at) = entry.expires_at {
            Ok(Instant::now() < expires_at)
        } else {
            Ok(true)
        }
    }
}
```

## Performance Optimization

### Query Optimization

```rust
pub struct QueryOptimizer {
    /// Optimization rules
    rules: Vec<Box<dyn OptimizationRule>>,
    /// Cost model
    cost_model: Box<dyn CostModel>,
    /// Statistics collector
    stats_collector: Box<dyn StatisticsCollector>,
}

impl QueryOptimizer {
    /// Optimize query
    pub fn optimize(&self, query: &Query) -> Result<Query, OptimizationError> {
        let mut optimized = query.clone();

        // Apply optimization rules
        for rule in &self.rules {
            optimized = rule.apply(&optimized)?;
        }

        // Cost-based optimization
        optimized = self.cost_based_optimization(&optimized)?;

        Ok(optimized)
    }

    /// Cost-based optimization
    fn cost_based_optimization(&self, query: &Query) -> Result<Query, OptimizationError> {
        // Generate alternative plans
        let alternatives = self.generate_alternatives(query)?;

        // Calculate costs
        let mut best_query = query.clone();
        let mut best_cost = f64::INFINITY;

        for alternative in alternatives {
            let cost = self.calculate_cost(&alternative)?;
            if cost < best_cost {
                best_cost = cost;
                best_query = alternative;
            }
        }

        Ok(best_query)
    }
}
```

## Monitoring and Metrics

### Query Metrics

```rust
#[derive(Debug, Clone)]
pub struct QueryMetrics {
    /// Query execution time
    pub execution_time: Duration,
    /// Rows processed
    pub rows_processed: u64,
    /// Rows returned
    pub rows_returned: u64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Index usage statistics
    pub index_usage: HashMap<String, IndexUsageStats>,
    /// Memory usage
    pub memory_usage: u64,
    /// CPU usage
    pub cpu_usage: f64,
}

#[derive(Debug, Clone)]
pub struct IndexUsageStats {
    /// Number of times index was used
    pub usage_count: u64,
    /// Average time saved by using index
    pub avg_time_saved: Duration,
    /// Index selectivity
    pub selectivity: f64,
}
```

This specification provides a comprehensive query engine architecture that ensures efficient, scalable, and powerful querying capabilities for Edda's AI agent task and document management system.
