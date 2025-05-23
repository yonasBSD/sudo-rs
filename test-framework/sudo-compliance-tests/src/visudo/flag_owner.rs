use sudo_test::{Command, Env, TextFile, ROOT_GROUP};

use crate::{
    visudo::{CHMOD_EXEC, DEFAULT_EDITOR, EDITOR_DUMMY, ETC_SUDOERS, TMP_SUDOERS},
    USERNAME,
};

#[test]
fn when_present_changes_ownership_of_existing_file() {
    let file_path = TMP_SUDOERS;
    let env = Env("")
        .file(file_path, TextFile("").chown("root:users").chmod("777"))
        .file(DEFAULT_EDITOR, TextFile(EDITOR_DUMMY).chmod(CHMOD_EXEC))
        .build();

    Command::new("visudo")
        .args(["--owner", "--file", file_path])
        .output(&env)
        .assert_success();

    let ls_output = Command::new("ls")
        .args(["-l", file_path])
        .output(&env)
        .stdout();

    assert_contains!(ls_output, format!(" root {ROOT_GROUP} "));
}

#[test]
fn when_absent_ownership_is_preserved() {
    let file_path = TMP_SUDOERS;
    let env = Env("")
        .file(file_path, TextFile("").chown("root:users").chmod("777"))
        .file(DEFAULT_EDITOR, TextFile(EDITOR_DUMMY).chmod(CHMOD_EXEC))
        .build();

    Command::new("visudo")
        .args(["--file", file_path])
        .output(&env)
        .assert_success();

    let ls_output = Command::new("ls")
        .args(["-l", file_path])
        .output(&env)
        .stdout();

    assert_contains!(ls_output, " root users ");
}

#[test]
fn etc_sudoers_ownership_is_always_changed() {
    let file_path = ETC_SUDOERS;
    let env = Env(TextFile("").chown(format!("{USERNAME}:users")).chmod("777"))
        .file(DEFAULT_EDITOR, TextFile(EDITOR_DUMMY).chmod(CHMOD_EXEC))
        .user(USERNAME)
        .build();

    Command::new("visudo").output(&env).assert_success();

    let ls_output = Command::new("ls")
        .args(["-l", file_path])
        .output(&env)
        .stdout();

    assert_contains!(ls_output, format!(" root {ROOT_GROUP} "));
}

#[test]
fn flag_check() {
    let file_path = TMP_SUDOERS;
    let env = Env("")
        .file(
            file_path,
            TextFile("").chown(format!("{USERNAME}:users")).chmod("777"),
        )
        .file(DEFAULT_EDITOR, TextFile(EDITOR_DUMMY).chmod(CHMOD_EXEC))
        .user(USERNAME)
        .build();

    let output = Command::new("visudo")
        .args(["--check", "--owner", "--file", file_path])
        .output(&env);

    output.assert_exit_code(1);
    assert_contains!(
        output.stderr(),
        format!("{file_path}: wrong owner (uid, gid) should be (0, 0)")
    );
}
