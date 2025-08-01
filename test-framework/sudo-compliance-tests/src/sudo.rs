#[cfg(feature = "apparmor")]
mod apparmor;
mod child_process;
mod cli;
mod env_reset;
mod flag_chdir;
mod flag_group;
mod flag_help;
mod flag_list;
mod flag_login;
mod flag_non_interactive;
mod flag_preserve_environment;
mod flag_prompt;
mod flag_shell;
mod flag_user;
mod flag_version;
mod lecture;
mod lecture_file;
mod limits;
mod misc;
mod nopasswd;
mod pam;
mod pass_auth;
mod passwd;
mod password_retry;
mod path_search;
mod perms;
mod sudo_ps1;
mod sudoers;
mod syslog;
mod timestamp;
mod umask;
mod use_pty;
