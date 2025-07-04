# Edda - Deployment Specification

## Overview

This specification defines the deployment architecture for Edda, providing comprehensive build systems, packaging, and distribution capabilities. The deployment system ensures reliable delivery across multiple platforms with automated packaging and distribution workflows.

## Architecture Context

This specification defines the deployment architecture that supports the Edda architecture as defined in [SPEC_ARCHITECTURE.md](./SPEC_ARCHITECTURE.md). The deployment system ensures reliable delivery across multiple platforms with automated packaging and distribution workflows for all architectural components.

## Development Workflow Integration

> **Note**: This specification focuses on deployment and distribution aspects. For development workflow, build system configuration, and testing strategies, see [SPEC_DEVELOPMENT.md](./SPEC_DEVELOPMENT.md). The deployment system integrates with the centralized development workflow defined there.

## Architecture Overview

### Deployment System Design

The deployment system supports all architectural layers defined in the master architecture:

- **CLI Interface Layer**: Deployment tools for CLI component packaging and distribution
- **Core Engine Layer**: Deployment tools for business logic component packaging and distribution
- **Data Storage Layer**: Deployment tools for storage component packaging and distribution
- **AI Agent Integration Layer**: Deployment tools for AI integration component packaging and distribution

## Core Deployment Components

### Deployment Manager

```rust
pub struct DeploymentManager {
    /// Build manager
    build_manager: Arc<BuildManager>,
    /// Package manager
    package_manager: Arc<PackageManager>,
    /// Distribution manager
    distribution_manager: Arc<DistributionManager>,
    /// Deployment configuration
    config: DeploymentConfig,
}

#[derive(Debug, Clone)]
pub struct DeploymentConfig {
    /// Target platforms
    pub target_platforms: Vec<Platform>,
    /// Build configuration
    pub build_config: BuildConfig,
    /// Package configuration
    pub package_config: PackageConfig,
    /// Distribution configuration
    pub distribution_config: DistributionConfig,
    /// Signing configuration
    pub signing_config: Option<SigningConfig>,
}

#[derive(Debug, Clone)]
pub enum Platform {
    /// Windows x86_64
    WindowsX86_64,
    /// Windows ARM64
    WindowsARM64,
    /// macOS x86_64
    MacOSX86_64,
    /// macOS ARM64
    MacOSARM64,
    /// Linux x86_64
    LinuxX86_64,
    /// Linux ARM64
    LinuxARM64,
    /// Linux ARMv7
    LinuxARMv7,
}

impl DeploymentManager {
    /// Deploy project
    pub fn deploy_project(&self, version: &str) -> Result<DeploymentResult, DeploymentError> {
        let mut result = DeploymentResult::new();

        // Build for all platforms
        let build_results = self.build_for_all_platforms(version)?;
        result.set_build_results(build_results);

        // Package artifacts
        let package_results = self.package_artifacts(version)?;
        result.set_package_results(package_results);

        // Sign packages
        if let Some(signing_config) = &self.config.signing_config {
            let signing_results = self.sign_packages(signing_config)?;
            result.set_signing_results(signing_results);
        }

        // Distribute packages
        let distribution_results = self.distribute_packages(version)?;
        result.set_distribution_results(distribution_results);

        // Update release notes
        self.update_release_notes(version)?;

        Ok(result)
    }

    /// Build for all platforms
    fn build_for_all_platforms(&self, version: &str) -> Result<Vec<PlatformBuildResult>, DeploymentError> {
        let mut results = Vec::new();

        for platform in &self.config.target_platforms {
            let result = self.build_for_platform(platform, version)?;
            results.push(result);
        }

        Ok(results)
    }

    /// Build for specific platform
    fn build_for_platform(&self, platform: &Platform, version: &str) -> Result<PlatformBuildResult, DeploymentError> {
        // Note: Build system configuration is centralized in SPEC_DEVELOPMENT.md
        // This method uses the standardized build system defined there

        // Set up cross-compilation target
        let target = self.get_target_triple(platform)?;

        // Use centralized build system from SPEC_DEVELOPMENT.md
        let build_manager = BuildManager::new()?;
        let build_result = build_manager.build_for_target(&target, version)?;

        // Collect artifacts
        let artifacts = self.collect_platform_artifacts(platform, &target)?;

        Ok(PlatformBuildResult {
            platform: platform.clone(),
            target: target,
            artifacts,
            success: build_result.success,
            build_time: build_result.build_time,
        })
    }

    /// Get target triple for platform
    fn get_target_triple(&self, platform: &Platform) -> Result<String, DeploymentError> {
        match platform {
            Platform::WindowsX86_64 => Ok("x86_64-pc-windows-msvc".to_string()),
            Platform::WindowsARM64 => Ok("aarch64-pc-windows-msvc".to_string()),
            Platform::MacOSX86_64 => Ok("x86_64-apple-darwin".to_string()),
            Platform::MacOSARM64 => Ok("aarch64-apple-darwin".to_string()),
            Platform::LinuxX86_64 => Ok("x86_64-unknown-linux-gnu".to_string()),
            Platform::LinuxARM64 => Ok("aarch64-unknown-linux-gnu".to_string()),
            Platform::LinuxARMv7 => Ok("armv7-unknown-linux-gnueabihf".to_string()),
        }
    }

    /// Collect platform artifacts
    fn collect_platform_artifacts(&self, platform: &Platform, target: &str) -> Result<Vec<BuildArtifact>, DeploymentError> {
        let mut artifacts = Vec::new();

        let target_dir = format!("target/{}/release", target);

        // Look for binary
        let binary_name = match platform {
            Platform::WindowsX86_64 | Platform::WindowsARM64 => "edda.exe",
            _ => "edda",
        };

        let binary_path = format!("{}/{}", target_dir, binary_name);

        if std::path::Path::new(&binary_path).exists() {
            let metadata = std::fs::metadata(&binary_path)?;

            artifacts.push(BuildArtifact {
                name: binary_name.to_string(),
                path: binary_path,
                size: metadata.len(),
                created_at: metadata.created()?.into(),
            });
        }

        Ok(artifacts)
    }
}
```

## Package Management

### Package Manager

```rust
pub struct PackageManager {
    /// Package configuration
    config: PackageConfig,
    /// Package formats
    formats: HashMap<PackageFormat, Box<dyn PackageFormatter>>,
    /// Package validators
    validators: Vec<Box<dyn PackageValidator>>,
}

#[derive(Debug, Clone)]
pub struct PackageConfig {
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
    /// Package description
    pub description: String,
    /// Package author
    pub author: String,
    /// Package license
    pub license: String,
    /// Package homepage
    pub homepage: Option<String>,
    /// Package repository
    pub repository: Option<String>,
    /// Package keywords
    pub keywords: Vec<String>,
    /// Package categories
    pub categories: Vec<String>,
    /// Package dependencies
    pub dependencies: Vec<PackageDependency>,
}

#[derive(Debug, Clone)]
pub enum PackageFormat {
    /// ZIP archive
    Zip,
    /// TAR.GZ archive
    TarGz,
    /// DEB package
    Deb,
    /// RPM package
    Rpm,
    /// MSI installer
    Msi,
    /// DMG installer
    Dmg,
    /// AppImage
    AppImage,
}

impl PackageManager {
    /// Create packages
    pub fn create_packages(&self, artifacts: &[BuildArtifact]) -> Result<Vec<Package>, PackageError> {
        let mut packages = Vec::new();

        for format in &self.config.formats {
            let package = self.create_package(format, artifacts)?;
            packages.push(package);
        }

        Ok(packages)
    }

    /// Create package for format
    fn create_package(&self, format: &PackageFormat, artifacts: &[BuildArtifact]) -> Result<Package, PackageError> {
        let formatter = self.formats.get(format)
            .ok_or(PackageError::UnsupportedFormat)?;

        // Create package
        let package = formatter.create_package(&self.config, artifacts)?;

        // Validate package
        for validator in &self.validators {
            validator.validate_package(&package)?;
        }

        Ok(package)
    }

    /// Create ZIP package
    fn create_zip_package(&self, config: &PackageConfig, artifacts: &[BuildArtifact]) -> Result<Package, PackageError> {
        let zip_path = format!("dist/{}-{}.zip", config.name, config.version);

        // Create ZIP file
        let file = std::fs::File::create(&zip_path)?;
        let mut zip = zip::ZipWriter::new(file);

        // Add artifacts to ZIP
        for artifact in artifacts {
            let mut file = std::fs::File::open(&artifact.path)?;
            zip.start_file(&artifact.name, zip::write::FileOptions::default())?;
            std::io::copy(&mut file, &mut zip)?;
        }

        // Add package metadata
        let metadata = serde_json::to_string(config)?;
        zip.start_file("package.json", zip::write::FileOptions::default())?;
        zip.write_all(metadata.as_bytes())?;

        zip.finish()?;

        Ok(Package {
            format: PackageFormat::Zip,
            path: zip_path,
            size: std::fs::metadata(&zip_path)?.len(),
            checksum: self.calculate_checksum(&zip_path)?,
        })
    }

    /// Create DEB package
    fn create_deb_package(&self, config: &PackageConfig, artifacts: &[BuildArtifact]) -> Result<Package, PackageError> {
        let deb_path = format!("dist/{}_{}_{}.deb", config.name, config.version, self.get_architecture()?);

        // Create DEB package structure
        let control_content = self.generate_deb_control(config)?;
        let postinst_content = self.generate_deb_postinst()?;

        // Create DEB package
        self.create_deb_file(&deb_path, &control_content, &postinst_content, artifacts)?;

        Ok(Package {
            format: PackageFormat::Deb,
            path: deb_path,
            size: std::fs::metadata(&deb_path)?.len(),
            checksum: self.calculate_checksum(&deb_path)?,
        })
    }

    /// Generate DEB control file
    fn generate_deb_control(&self, config: &PackageConfig) -> Result<String, PackageError> {
        let control = format!(
            "Package: {}\n\
             Version: {}\n\
             Architecture: {}\n\
             Maintainer: {}\n\
             Description: {}\n\
             Homepage: {}\n\
             Section: utils\n\
             Priority: optional\n",
            config.name,
            config.version,
            self.get_architecture()?,
            config.author,
            config.description,
            config.homepage.as_deref().unwrap_or(""),
        );

        Ok(control)
    }

    /// Get architecture for package
    fn get_architecture(&self) -> Result<String, PackageError> {
        let output = std::process::Command::new("dpkg")
            .arg("--print-architecture")
            .output()?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Ok("amd64".to_string()) // Default fallback
        }
    }
}
```

## Distribution Management

### Distribution Manager

```rust
pub struct DistributionManager {
    /// Distribution configuration
    config: DistributionConfig,
    /// Distribution channels
    channels: HashMap<DistributionChannel, Box<dyn DistributionChannel>>,
    /// Release manager
    release_manager: Arc<ReleaseManager>,
}

#[derive(Debug, Clone)]
pub struct DistributionConfig {
    /// GitHub configuration
    pub github: GitHubConfig,
    /// Package manager configuration
    pub package_managers: PackageManagerConfig,
    /// Container registry configuration
    pub container_registry: ContainerRegistryConfig,
    /// Release notes template
    pub release_notes_template: String,
}

#[derive(Debug, Clone)]
pub enum DistributionChannel {
    /// GitHub Releases
    GitHubReleases,
    /// Homebrew
    Homebrew,
    /// Chocolatey
    Chocolatey,
    /// Snapcraft
    Snapcraft,
    /// Docker Hub
    DockerHub,
    /// GitHub Container Registry
    GitHubContainerRegistry,
}

impl DistributionManager {
    /// Distribute packages
    pub fn distribute_packages(&self, version: &str) -> Result<Vec<DistributionResult>, DistributionError> {
        let mut results = Vec::new();

        // Get packages for distribution
        let packages = self.get_packages_for_distribution(version)?;

        // Distribute to each channel
        for (channel, channel_impl) in &self.channels {
            let result = channel_impl.distribute(&packages, version)?;
            results.push(result);
        }

        // Create GitHub release
        let release_result = self.create_github_release(version, &packages)?;
        results.push(release_result);

        // Update package managers
        let package_manager_results = self.update_package_managers(version, &packages)?;
        results.extend(package_manager_results);

        Ok(results)
    }

    /// Create GitHub release
    fn create_github_release(&self, version: &str, packages: &[Package]) -> Result<DistributionResult, DistributionError> {
        let release_notes = self.generate_release_notes(version)?;

        // Create release on GitHub
        let client = reqwest::Client::new();
        let token = std::env::var("GITHUB_TOKEN")
            .map_err(|_| DistributionError::MissingToken)?;

        let release_data = serde_json::json!({
            "tag_name": format!("v{}", version),
            "name": format!("Edda {}", version),
            "body": release_notes,
            "draft": false,
            "prerelease": version.contains("alpha") || version.contains("beta"),
        });

        let response = client
            .post(&format!("https://api.github.com/repos/{}/releases", self.config.github.repository))
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "edda-deployment")
            .json(&release_data)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(DistributionError::GitHubReleaseFailed(response.text().await?));
        }

        let release: serde_json::Value = response.json().await?;
        let release_id = release["id"].as_u64().unwrap();

        // Upload assets
        for package in packages {
            self.upload_github_asset(release_id, package).await?;
        }

        Ok(DistributionResult {
            channel: DistributionChannel::GitHubReleases,
            success: true,
            url: Some(format!("https://github.com/{}/releases/tag/v{}", self.config.github.repository, version)),
            error: None,
        })
    }

    /// Upload GitHub asset
    async fn upload_github_asset(&self, release_id: u64, package: &Package) -> Result<(), DistributionError> {
        let client = reqwest::Client::new();
        let token = std::env::var("GITHUB_TOKEN")
            .map_err(|_| DistributionError::MissingToken)?;

        let file_content = std::fs::read(&package.path)?;
        let filename = std::path::Path::new(&package.path)
            .file_name()
            .unwrap()
            .to_string_lossy();

        let response = client
            .post(&format!("https://uploads.github.com/repos/{}/releases/{}/assets?name={}",
                self.config.github.repository, release_id, filename))
            .header("Authorization", format!("token {}", token))
            .header("Content-Type", "application/octet-stream")
            .body(file_content)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(DistributionError::AssetUploadFailed(response.text().await?));
        }

        Ok(())
    }

    /// Generate release notes
    fn generate_release_notes(&self, version: &str) -> Result<String, DistributionError> {
        // Get commits since last release
        let commits = self.get_commits_since_last_release()?;

        // Categorize commits
        let features = commits.iter().filter(|c| c.message.contains("feat:")).collect::<Vec<_>>();
        let fixes = commits.iter().filter(|c| c.message.contains("fix:")).collect::<Vec<_>>();
        let breaking = commits.iter().filter(|c| c.message.contains("BREAKING")).collect::<Vec<_>>();

        let mut notes = format!("# Edda {}\n\n", version);

        if !breaking.is_empty() {
            notes.push_str("## ⚠️ Breaking Changes\n\n");
            for commit in breaking {
                notes.push_str(&format!("- {}\n", commit.message));
            }
            notes.push_str("\n");
        }

        if !features.is_empty() {
            notes.push_str("## ✨ New Features\n\n");
            for commit in features {
                notes.push_str(&format!("- {}\n", commit.message));
            }
            notes.push_str("\n");
        }

        if !fixes.is_empty() {
            notes.push_str("## 🐛 Bug Fixes\n\n");
            for commit in fixes {
                notes.push_str(&format!("- {}\n", commit.message));
            }
            notes.push_str("\n");
        }

        notes.push_str("## 📦 Downloads\n\n");
        notes.push_str("Download the appropriate package for your platform:\n\n");

        Ok(notes)
    }
}
```

## Container Management

### Container Manager

```rust
pub struct ContainerManager {
    /// Container configuration
    config: ContainerConfig,
    /// Docker client
    docker_client: DockerClient,
    /// Container registry
    registry: ContainerRegistry,
}

#[derive(Debug, Clone)]
pub struct ContainerConfig {
    /// Base image
    pub base_image: String,
    /// Container ports
    pub ports: Vec<u16>,
    /// Container volumes
    pub volumes: Vec<String>,
    /// Environment variables
    pub environment: HashMap<String, String>,
    /// Container labels
    pub labels: HashMap<String, String>,
}

impl ContainerManager {
    /// Build container image
    pub fn build_container(&self, version: &str) -> Result<ContainerImage, ContainerError> {
        // Create Dockerfile
        let dockerfile = self.generate_dockerfile(version)?;

        // Build image
        let image_name = format!("edda:{}", version);
        let image_tag = format!("edda:{}", version);

        let output = std::process::Command::new("docker")
            .args(&["build", "-t", &image_name, "-t", &image_tag, "."])
            .output()?;

        if !output.status.success() {
            return Err(ContainerError::BuildFailed(String::from_utf8_lossy(&output.stderr).to_string()));
        }

        // Get image info
        let image_info = self.get_image_info(&image_name)?;

        Ok(ContainerImage {
            name: image_name,
            tag: image_tag,
            size: image_info.size,
            digest: image_info.digest,
            created_at: image_info.created_at,
        })
    }

    /// Push container image
    pub fn push_container(&self, image: &ContainerImage) -> Result<(), ContainerError> {
        // Tag for registry
        let registry_image = format!("{}/{}", self.config.registry.url, image.name);

        let status = std::process::Command::new("docker")
            .args(&["tag", &image.name, &registry_image])
            .status()?;

        if !status.success() {
            return Err(ContainerError::TagFailed);
        }

        // Push to registry
        let status = std::process::Command::new("docker")
            .args(&["push", &registry_image])
            .status()?;

        if !status.success() {
            return Err(ContainerError::PushFailed);
        }

        Ok(())
    }

    /// Generate Dockerfile
    fn generate_dockerfile(&self, version: &str) -> Result<String, ContainerError> {
        let dockerfile = format!(
            "FROM {}\n\
             \n\
             LABEL maintainer=\"{}\"\n\
             LABEL version=\"{}\"\n\
             LABEL description=\"{}\"\n\
             \n\
             # Install dependencies\n\
             RUN apt-get update && apt-get install -y \\\n\
                 ca-certificates \\\n\
                 && rm -rf /var/lib/apt/lists/*\n\
             \n\
             # Copy binary\n\
             COPY target/release/edda /usr/local/bin/edda\n\
             \n\
             # Set permissions\n\
             RUN chmod +x /usr/local/bin/edda\n\
             \n\
             # Create user\n\
             RUN useradd -r -s /bin/false edda\n\
             \n\
             # Switch to user\n\
             USER edda\n\
             \n\
             # Set entrypoint\n\
             ENTRYPOINT [\"/usr/local/bin/edda\"]\n\
             \n\
             # Expose ports\n\
             EXPOSE {}\n",
            self.config.base_image,
            self.config.maintainer,
            version,
            self.config.description,
            self.config.ports.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(" "),
        );

        // Write Dockerfile
        std::fs::write("Dockerfile", dockerfile)?;

        Ok(dockerfile)
    }
}
```

## Signing and Verification

### Signing Manager

```rust
pub struct SigningManager {
    /// Signing configuration
    config: SigningConfig,
    /// Signing keys
    keys: HashMap<String, SigningKey>,
}

#[derive(Debug, Clone)]
pub struct SigningConfig {
    /// GPG key ID
    pub gpg_key_id: String,
    /// GPG passphrase
    pub gpg_passphrase: Option<String>,
    /// Code signing certificate
    pub code_signing_cert: Option<String>,
    /// Code signing password
    pub code_signing_password: Option<String>,
}

impl SigningManager {
    /// Sign packages
    pub fn sign_packages(&self, packages: &[Package]) -> Result<Vec<SignedPackage>, SigningError> {
        let mut signed_packages = Vec::new();

        for package in packages {
            let signed_package = self.sign_package(package)?;
            signed_packages.push(signed_package);
        }

        Ok(signed_packages)
    }

    /// Sign package
    fn sign_package(&self, package: &Package) -> Result<SignedPackage, SigningError> {
        // Create signature file
        let signature_path = format!("{}.sig", package.path);

        // Sign with GPG
        let mut command = std::process::Command::new("gpg");
        command.args(&["--detach-sign", "--armor", "--output", &signature_path]);

        if let Some(passphrase) = &self.config.gpg_passphrase {
            command.env("GPG_PASSPHRASE", passphrase);
        }

        command.arg(&package.path);

        let status = command.status()?;

        if !status.success() {
            return Err(SigningError::GPGSigningFailed);
        }

        // Verify signature
        let verify_status = std::process::Command::new("gpg")
            .args(&["--verify", &signature_path, &package.path])
            .status()?;

        if !verify_status.success() {
            return Err(SigningError::SignatureVerificationFailed);
        }

        Ok(SignedPackage {
            package: package.clone(),
            signature_path,
            signed_at: Utc::now(),
        })
    }

    /// Verify package signature
    pub fn verify_package_signature(&self, package: &Package, signature_path: &str) -> Result<bool, SigningError> {
        let status = std::process::Command::new("gpg")
            .args(&["--verify", signature_path, &package.path])
            .status()?;

        Ok(status.success())
    }
}
```

## Monitoring and Metrics

### Deployment Metrics

```rust
#[derive(Debug, Clone)]
pub struct DeploymentMetrics {
    /// Build metrics
    pub build_metrics: BuildMetrics,
    /// Package metrics
    pub package_metrics: PackageMetrics,
    /// Distribution metrics
    pub distribution_metrics: DistributionMetrics,
    /// Container metrics
    pub container_metrics: ContainerMetrics,
}

#[derive(Debug, Clone)]
pub struct BuildMetrics {
    /// Total builds
    pub total_builds: u64,
    /// Successful builds
    pub successful_builds: u64,
    /// Failed builds
    pub failed_builds: u64,
    /// Average build time
    pub avg_build_time: Duration,
    /// Build success rate
    pub build_success_rate: f64,
    /// Cross-compilation success rate
    pub cross_compilation_success_rate: f64,
}

#[derive(Debug, Clone)]
pub struct PackageMetrics {
    /// Total packages created
    pub total_packages: u64,
    /// Package formats
    pub package_formats: HashMap<PackageFormat, u64>,
    /// Average package size
    pub avg_package_size: u64,
    /// Package creation time
    pub avg_package_creation_time: Duration,
}

#[derive(Debug, Clone)]
pub struct DistributionMetrics {
    /// Total distributions
    pub total_distributions: u64,
    /// Distribution channels
    pub distribution_channels: HashMap<DistributionChannel, u64>,
    /// Download counts
    pub download_counts: HashMap<String, u64>,
    /// Distribution success rate
    pub distribution_success_rate: f64,
}

#[derive(Debug, Clone)]
pub struct ContainerMetrics {
    /// Total container builds
    pub total_container_builds: u64,
    /// Container image sizes
    pub container_image_sizes: HashMap<String, u64>,
    /// Container registry pushes
    pub container_registry_pushes: u64,
    /// Container pull counts
    pub container_pull_counts: HashMap<String, u64>,
}
```

This specification provides a comprehensive deployment system that ensures reliable delivery across multiple platforms with automated packaging and distribution workflows.
