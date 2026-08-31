use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

const NXUS_CONFIG: &str = r#"[project]
default_profile = "sim"

[build]
root = "build"
link_compile_commands = true

[workspace]
root = "workspace"

[workspace.nuttx]

[workspace.nuttx_apps]

[profile.sim]
arch = "sim"
family = "sim"
board = "sim"
config_base = "nsh"

[profile.test]
arch = "sim"
family = "sim"
board = "sim"
config_base = "tests"

[profile.prod]
arch = "arm"
family = "stm32f7"
board = "nucleo-f767zi"
config_base = "evalos"

[profile.prod.flash]
command = "openocd"
args = ["-f", "board/st_nucleo_f7.cfg", "-c", "program {elf} verify reset exit"]
"#;

struct ProjectFixture {
    _temp_dir: TempDir,
    project_dir: PathBuf,
    app_dir: PathBuf,
    nested_dir: PathBuf,
}

impl ProjectFixture {
    fn new() -> Self {
        let temp_dir = TempDir::new().expect("tempdir should be created");
        let project_dir = temp_dir.path().join("project");
        let app_dir = project_dir.join("app");
        let nested_dir = app_dir.join("src").join("module");

        fs::create_dir_all(&nested_dir).expect("nested app directory should be created");
        write_file(&project_dir.join("nxus.toml"), NXUS_CONFIG);

        Self {
            _temp_dir: temp_dir,
            project_dir,
            app_dir,
            nested_dir,
        }
    }

    fn command(&self) -> Command {
        let mut command = Command::cargo_bin("nxus").expect("nxus binary should build");
        command.current_dir(&self.app_dir);
        command
    }

    fn command_from_nested_dir(&self) -> Command {
        let mut command = Command::cargo_bin("nxus").expect("nxus binary should build");
        command.current_dir(&self.nested_dir);
        command
    }

    fn write_app_config(&self, name: &str, content: &str) {
        write_file(&self.app_dir.join("config").join(name), content);
    }

    fn build_dir(&self, profile: &str) -> PathBuf {
        self.project_dir.join("build").join(profile)
    }

    fn firmware_elf(&self, profile: &str) -> PathBuf {
        self.build_dir(profile).join("nuttx")
    }

    fn workspace_root(&self) -> PathBuf {
        self.project_dir.join("workspace")
    }

    fn nuttx_dir(&self) -> PathBuf {
        self.workspace_root().join("nuttx")
    }

    fn nuttx_apps_dir(&self) -> PathBuf {
        self.workspace_root().join("nuttx-apps")
    }

    fn app_link(&self) -> PathBuf {
        self.nuttx_apps_dir().join("external")
    }

    fn generated_config_dir(&self, profile: &str) -> PathBuf {
        self.workspace_root().join("config").join(profile)
    }

    fn generated_config_file(&self, profile: &str) -> PathBuf {
        self.generated_config_dir(profile).join("defconfig")
    }

    fn board_config_root(&self, arch: &str, family: &str, board: &str) -> PathBuf {
        self.nuttx_dir()
            .join("boards")
            .join(arch)
            .join(family)
            .join(board)
            .join("configs")
    }

    fn board_config_base(
        &self,
        arch: &str,
        family: &str,
        board: &str,
        config_base: &str,
    ) -> PathBuf {
        self.board_config_root(arch, family, board)
            .join(config_base)
            .join("defconfig")
    }

    fn board_config_link(&self, arch: &str, family: &str, board: &str, profile: &str) -> PathBuf {
        self.board_config_root(arch, family, board).join(profile)
    }

    fn prepare_workspace_repos(&self) {
        fs::create_dir_all(self.nuttx_dir()).expect("nuttx dir should be created");
        fs::create_dir_all(self.nuttx_apps_dir()).expect("nuttx-apps dir should be created");
    }

    fn prepare_board_configs(&self) {
        write_file(
            &self.board_config_base("sim", "sim", "sim", "nsh"),
            "CONFIG_SIM=y\n",
        );
        write_file(
            &self.board_config_base("sim", "sim", "sim", "tests"),
            "CONFIG_TEST=y\n",
        );
        write_file(
            &self.board_config_base("arm", "stm32f7", "nucleo-f767zi", "evalos"),
            "CONFIG_PROD=y\n",
        );
    }

    fn prepare_config_inputs(&self) {
        self.write_app_config("common.config", "CONFIG_COMMON=y\n");
        self.write_app_config("sim.overlay", "CONFIG_SIM_OVERLAY=y\n");
        self.write_app_config("test.overlay", "CONFIG_TEST_OVERLAY=y\n");
        self.write_app_config("prod.overlay", "CONFIG_PROD_OVERLAY=y\n");
    }

    fn prepare_config_command(&self) {
        self.prepare_workspace_repos();
        self.prepare_board_configs();
        self.prepare_config_inputs();
    }

    fn prepare_app_link(&self) {
        fs::create_dir_all(
            self.app_link()
                .parent()
                .expect("app link parent should exist"),
        )
        .expect("app link parent should be created");
        unix_fs::symlink(&self.app_dir, self.app_link()).expect("app link should be created");
    }

    fn prepare_profile_link(&self, arch: &str, family: &str, board: &str, profile: &str) {
        let link_path = self.board_config_link(arch, family, board, profile);
        let target = self.generated_config_dir(profile);

        fs::create_dir_all(&target).expect("generated config dir should be created");
        fs::create_dir_all(link_path.parent().expect("link parent should exist"))
            .expect("link parent should be created");
        unix_fs::symlink(target, link_path).expect("profile link should be created");
    }

    fn prepare_workspace_prune(&self) {
        self.prepare_workspace_repos();
        fs::create_dir_all(self.nuttx_dir().join(".git")).expect("nuttx git dir should exist");
        fs::create_dir_all(self.nuttx_apps_dir().join(".git"))
            .expect("nuttx-apps git dir should exist");
        self.prepare_app_link();
        self.prepare_profile_link("sim", "sim", "sim", "sim");
        self.prepare_profile_link("sim", "sim", "sim", "test");
        self.prepare_profile_link("arm", "stm32f7", "nucleo-f767zi", "prod");
    }
}

fn write_file(path: &Path, content: &str) {
    fs::create_dir_all(path.parent().expect("file parent should exist"))
        .expect("file parent should be created");
    fs::write(path, content).expect("file should be written");
}

fn assert_ordered(stderr: &str, first: &str, second: &str) {
    let first_pos = stderr
        .find(first)
        .unwrap_or_else(|| panic!("`{first}` missing from stderr: {stderr}"));
    let second_pos = stderr
        .find(second)
        .unwrap_or_else(|| panic!("`{second}` missing from stderr: {stderr}"));

    assert!(
        first_pos < second_pos,
        "expected `{first}` before `{second}` in stderr: {stderr}"
    );
}

#[test]
fn profiles_lists_profiles_when_invoked_from_nested_directory() {
    let fixture = ProjectFixture::new();

    fixture
        .command_from_nested_dir()
        .arg("profiles")
        .assert()
        .success()
        .stdout(predicate::str::contains("Profile"))
        .stdout(predicate::str::contains("sim"))
        .stdout(predicate::str::contains("prod"));
}

#[test]
fn missing_config_fails_with_clear_error() {
    let temp_dir = TempDir::new().expect("tempdir should be created");

    let mut command = Command::cargo_bin("nxus").expect("nxus binary should build");
    command.current_dir(temp_dir.path());

    command
        .arg("profiles")
        .assert()
        .failure()
        .stderr(predicate::str::contains("`nxus.toml` config not found"));
}

#[test]
fn invalid_config_fails_with_parse_error() {
    let fixture = ProjectFixture::new();
    write_file(&fixture.project_dir.join("nxus.toml"), "not = [\n");

    fixture
        .command()
        .arg("profiles")
        .assert()
        .failure()
        .stderr(predicate::str::contains("failed to parse config file"));
}

#[test]
fn unknown_profile_fails_during_resolution() {
    let fixture = ProjectFixture::new();

    fixture
        .command()
        .args(["-p", "missing", "profiles"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown profile: `missing`"));
}

#[test]
fn clean_without_explicit_profile_removes_workspace_links_and_build_root() {
    let fixture = ProjectFixture::new();

    fs::create_dir_all(fixture.build_dir("sim")).expect("sim build dir should be created");
    fs::create_dir_all(fixture.build_dir("prod")).expect("prod build dir should be created");
    fixture.prepare_workspace_prune();

    fixture.command().arg("clean").assert().success();

    assert!(!fixture.project_dir.join("build").exists());
    assert!(!fixture.app_link().exists());
    assert!(
        !fixture
            .board_config_link("sim", "sim", "sim", "sim")
            .exists()
    );
    assert!(
        !fixture
            .board_config_link("sim", "sim", "sim", "test")
            .exists()
    );
    assert!(
        !fixture
            .board_config_link("arm", "stm32f7", "nucleo-f767zi", "prod")
            .exists()
    );
}

#[test]
fn clean_alias_with_explicit_profile_removes_only_selected_build_dir() {
    let fixture = ProjectFixture::new();

    fs::create_dir_all(fixture.build_dir("sim")).expect("sim build dir should be created");
    fs::create_dir_all(fixture.build_dir("prod")).expect("prod build dir should be created");
    fixture.prepare_profile_link("sim", "sim", "sim", "sim");

    fixture
        .command()
        .args(["-p", "sim", "c"])
        .assert()
        .success();

    assert!(!fixture.build_dir("sim").exists());
    assert!(fixture.build_dir("prod").exists());
    assert!(
        !fixture
            .board_config_link("sim", "sim", "sim", "sim")
            .exists()
    );
}

#[test]
fn config_creates_links_generated_config_and_build_dir() {
    let fixture = ProjectFixture::new();
    fixture.prepare_config_command();

    fixture
        .command()
        .args(["-d", "-p", "prod", "config"])
        .assert()
        .success();

    assert!(fixture.build_dir("prod").exists());
    assert!(fixture.app_link().exists());
    assert!(fixture.generated_config_file("prod").is_file());
    assert!(
        fixture
            .board_config_link("arm", "stm32f7", "nucleo-f767zi", "prod")
            .exists()
    );
}

#[test]
fn build_alias_honors_global_flags_and_prints_ninja_command() {
    let fixture = ProjectFixture::new();

    fs::create_dir_all(fixture.build_dir("prod")).expect("prod build dir should be created");

    fixture
        .command()
        .args(["-d", "-vvv", "-p", "prod", "b"])
        .assert()
        .success()
        .stderr(predicate::str::contains("ninja -C"))
        .stderr(predicate::str::contains("build/prod"));
}

#[test]
fn build_fails_when_build_path_is_a_file() {
    let fixture = ProjectFixture::new();

    write_file(&fixture.build_dir("sim"), "file");

    fixture
        .command()
        .arg("build")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "already exists and is not a directory",
        ));
}

#[test]
fn menuconfig_alias_prints_menuconfig_command() {
    let fixture = ProjectFixture::new();

    fs::create_dir_all(fixture.build_dir("prod")).expect("prod build dir should be created");

    fixture
        .command()
        .args(["-d", "-vvv", "-p", "prod", "m"])
        .assert()
        .success()
        .stderr(predicate::str::contains("menuconfig"));
}

#[test]
fn run_alias_prints_selected_profile_binary_path() {
    let fixture = ProjectFixture::new();

    fs::create_dir_all(fixture.build_dir("prod")).expect("prod build dir should be created");

    let assert = fixture
        .command()
        .args(["-d", "-vvv", "-p", "prod", "r"])
        .assert()
        .success();

    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(stderr.contains("ninja -C"));
    assert!(stderr.contains("build/prod/nuttx"));
    assert_ordered(&stderr, "ninja -C", "build/prod/nuttx");
}

#[test]
fn run_skips_rebuild_when_firmware_exists_and_rebuild_is_not_requested() {
    let fixture = ProjectFixture::new();

    fs::create_dir_all(fixture.build_dir("prod")).expect("prod build dir should be created");
    write_file(&fixture.firmware_elf("prod"), "elf\n");

    let assert = fixture
        .command()
        .args(["-d", "-vvv", "-p", "prod", "run"])
        .assert()
        .success();

    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(!stderr.contains("ninja -C"));
    assert!(stderr.contains("build/prod/nuttx"));
}

#[test]
fn run_rebuilds_when_requested_even_if_firmware_exists() {
    let fixture = ProjectFixture::new();

    fs::create_dir_all(fixture.build_dir("prod")).expect("prod build dir should be created");
    write_file(&fixture.firmware_elf("prod"), "elf\n");

    let assert = fixture
        .command()
        .args(["-d", "-vvv", "--rebuild", "-p", "prod", "run"])
        .assert()
        .success();

    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(stderr.contains("ninja -C"));
    assert!(stderr.contains("build/prod/nuttx"));
    assert_ordered(&stderr, "ninja -C", "build/prod/nuttx");
}

#[test]
fn sim_command_uses_sim_profile_binary_path() {
    let fixture = ProjectFixture::new();

    fs::create_dir_all(fixture.build_dir("sim")).expect("sim build dir should be created");

    fixture
        .command()
        .args(["-d", "-vvv", "sim"])
        .assert()
        .success()
        .stderr(predicate::str::contains("build/sim/nuttx"));
}

#[test]
fn sim_rebuild_flag_propagates_to_run_behavior() {
    let fixture = ProjectFixture::new();

    fs::create_dir_all(fixture.build_dir("sim")).expect("sim build dir should be created");
    write_file(&fixture.firmware_elf("sim"), "elf\n");

    let assert = fixture
        .command()
        .args(["-d", "-vvv", "--rebuild", "sim"])
        .assert()
        .success();

    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(stderr.contains("ninja -C"));
    assert!(stderr.contains("build/sim/nuttx"));
    assert_ordered(&stderr, "ninja -C", "build/sim/nuttx");
}

#[test]
fn test_alias_uses_test_profile_binary_path() {
    let fixture = ProjectFixture::new();

    fs::create_dir_all(fixture.build_dir("test")).expect("test build dir should be created");

    fixture
        .command()
        .args(["-d", "-vvv", "t"])
        .assert()
        .success()
        .stderr(predicate::str::contains("build/test/nuttx"));
}

#[test]
fn workspace_init_succeeds_with_existing_repo_directories() {
    let fixture = ProjectFixture::new();
    fixture.prepare_workspace_repos();

    fixture
        .command()
        .args(["-d", "workspace", "init"])
        .assert()
        .success();
}

#[test]
fn workspace_clean_alias_removes_workspace_root() {
    let fixture = ProjectFixture::new();

    fs::create_dir_all(fixture.workspace_root()).expect("workspace root should be created");

    fixture
        .command()
        .args(["workspace", "c"])
        .assert()
        .success();

    assert!(!fixture.workspace_root().exists());
}

#[test]
fn workspace_prune_alias_stashes_and_unlinks() {
    let fixture = ProjectFixture::new();
    fixture.prepare_workspace_prune();

    fixture.command().args(["-d", "ws", "p"]).assert().success();

    assert!(!fixture.app_link().exists());
    assert!(
        !fixture
            .board_config_link("sim", "sim", "sim", "sim")
            .exists()
    );
    assert!(
        !fixture
            .board_config_link("sim", "sim", "sim", "test")
            .exists()
    );
    assert!(
        !fixture
            .board_config_link("arm", "stm32f7", "nucleo-f767zi", "prod")
            .exists()
    );
}

#[test]
fn init_config_succeeds_without_existing_project() {
    let temp_dir = TempDir::new().expect("tempdir should be created");

    let mut command = Command::cargo_bin("nxus").expect("nxus binary should build");
    command.current_dir(temp_dir.path());

    command.args(["init", "config"]).assert().success();

    assert!(temp_dir.path().join("nxus.toml").is_file());
    assert!(temp_dir.path().join("config/common.config").is_file());

    let mut profiles = Command::cargo_bin("nxus").expect("nxus binary should build");
    profiles.current_dir(temp_dir.path());
    profiles
        .arg("profiles")
        .assert()
        .success()
        .stdout(predicate::str::contains("sim"))
        .stdout(predicate::str::contains("test"));
}

#[test]
fn init_config_refuses_to_overwrite_existing_config() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    write_file(&temp_dir.path().join("nxus.toml"), "existing\n");

    let mut command = Command::cargo_bin("nxus").expect("nxus binary should build");
    command.current_dir(temp_dir.path());

    command
        .args(["init", "config"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("refusing to overwrite"));
}

#[test]
fn init_project_creates_new_project_scaffold() {
    let temp_dir = TempDir::new().expect("tempdir should be created");

    let mut command = Command::cargo_bin("nxus").expect("nxus binary should build");
    command.current_dir(temp_dir.path());

    command.args(["init", "project", "demo"]).assert().success();

    let project_dir = temp_dir.path().join("demo");
    assert!(project_dir.join("nxus.toml").is_file());
    assert!(project_dir.join("app/CMakeLists.txt").is_file());
    assert!(project_dir.join("app/Kconfig").is_file());
    assert!(project_dir.join("app/config/common.config").is_file());

    let mut profiles = Command::cargo_bin("nxus").expect("nxus binary should build");
    profiles.current_dir(project_dir.join("app"));
    profiles
        .arg("profiles")
        .assert()
        .success()
        .stdout(predicate::str::contains("sim"));
}

#[test]
fn init_project_refuses_non_empty_destination() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let project_dir = temp_dir.path().join("demo");
    fs::create_dir_all(&project_dir).expect("project dir should be created");
    write_file(&project_dir.join("README.md"), "existing\n");

    let mut command = Command::cargo_bin("nxus").expect("nxus binary should build");
    command.current_dir(temp_dir.path());

    command
        .args(["init", "project", "demo"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("must be empty"));
}

#[test]
fn flash_uses_profile_command_configuration_in_dry_run() {
    let fixture = ProjectFixture::new();

    fs::create_dir_all(fixture.build_dir("prod")).expect("prod build dir should be created");
    write_file(&fixture.firmware_elf("prod"), "elf\n");

    let assert = fixture
        .command()
        .args(["-d", "-vvv", "-p", "prod", "flash"])
        .assert()
        .success();

    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(!stderr.contains("ninja -C"));
    assert!(stderr.contains("openocd"));
    assert!(stderr.contains("program"));
    assert!(stderr.contains("build/prod/nuttx"));
}

#[test]
fn flash_rebuilds_when_requested_even_if_firmware_exists() {
    let fixture = ProjectFixture::new();

    fs::create_dir_all(fixture.build_dir("prod")).expect("prod build dir should be created");
    write_file(&fixture.firmware_elf("prod"), "elf\n");

    let assert = fixture
        .command()
        .args(["-d", "-vvv", "--rebuild", "-p", "prod", "flash"])
        .assert()
        .success();

    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(stderr.contains("ninja -C"));
    assert!(stderr.contains("openocd"));
    assert!(stderr.contains("build/prod/nuttx"));
    assert_ordered(&stderr, "ninja -C", "openocd");
}

#[test]
fn flash_fails_when_profile_has_no_flash_configuration() {
    let fixture = ProjectFixture::new();

    fs::create_dir_all(fixture.build_dir("sim")).expect("sim build dir should be created");

    fixture
        .command()
        .args(["-d", "-p", "sim", "flash"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("does not define a flash command"));
}
