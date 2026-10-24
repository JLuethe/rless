# Observed option defaults

These C-locale queries were captured from the frozen normal reference. Startup uses the observer's explicit prompt/keypad/paste flags; those overrides are identified below. Other defaults come from the untouched option state. A query diagnostic and its display text are part of the contract.

| Option | Query result | Observer override |
| --- | --- | --- |
| `--search-skip-screen` | Search includes all of displayed screen  (press RETURN) | no |
| `--buffers` | Max buffer space per file: 64K  (press RETURN) | no |
| `--auto-buffers` | Automatically allocate buffers when needed  (press RETURN) | no |
| `--clear-screen` | Repaint by scrolling from bottom of screen  (press RETURN) | no |
| `--dumb` | Assume intelligent terminal  (press RETURN) | no |
| `--quit-at-eof` | Don't quit at end-of-file  (press RETURN) | no |
| `--force` | Open only regular files  (press RETURN) | no |
| `--quit-if-one-screen` | Don't quit if end-of-file on first screen  (press RETURN) | no |
| `--hilite-search` | Highlight all matches for previous search pattern  (press RETURN) | no |
| `--max-back-scroll` | Backwards scroll limit is -1 lines  (press RETURN) | no |
| `--ignore-case` | Case is significant in searches  (press RETURN) | no |
| `--jump-target` | Position target at screen line 0  (press RETURN) | no |
| `--status-column` | Don't display a status column  (press RETURN) | no |
| `--quit-on-intr` | Don't quit if end-of-file on first screen  (press RETURN) | no |
| `--no-lessopen` | Use the LESSOPEN filter  (press RETURN) | no |
| `--long-prompt` | Short prompt  (press RETURN) | no |
| `--line-numbers` | Use line numbers  (press RETURN) | no |
| `--log-file` | No log file  (press RETURN) | no |
| `--LOG-FILE` | No log file  (press RETURN) | no |
| `--prompt` | Prompt (short): @@READY@@  (press RETURN) | yes |
| `--quiet` | Ring the bell for errors AND at eof/bof  (press RETURN) | no |
| `--raw-control-chars` | Display ANSI sequences directly, other control characters as ^X  (press RETURN) | no |
| `--squeeze-blank-lines` | Display all blank lines  (press RETURN) | no |
| `--chop-long-lines` | Fold long lines  (press RETURN) | no |
| `--tag-file` | Cannot query the -t (--tag) option  (press RETURN) | no |
| `--underline-special` | Display underlined text in underline mode  (press RETURN) | no |
| `--hilite-target` | Don't highlight target line  (press RETURN) | no |
| `--hilite-unread` | Don't highlight first unread line  (press RETURN) | no |
| `--tabs` | Tab stops every 8 spaces  (press RETURN) | no |
| `--no-init` | Don't use init/deinit strings  (press RETURN) | yes |
| `--max-forw-scroll` | Forward scroll limit is -1 lines  (press RETURN) | no |
| `--window` | Scroll window size is -1 lines  (press RETURN) | no |
| `--quotes` | quotes ""  (press RETURN) | no |
| `--tilde` | Show tildes after end of file  (press RETURN) | no |
| `--shift` | Horizontal shift 0 columns  (press RETURN) | no |
| `--no-keypad` | Don't use keypad mode  (press RETURN) | yes |
| `--old-bot` | Use new bottom of screen behavior  (press RETURN) | no |
| `--follow-name` | F command follows file descriptor  (press RETURN) | no |
| `--use-backslash` | Don't use backslash escaping in command line parameters  (press RETURN) | no |
| `--rscroll` | rscroll character is >  (press RETURN) | no |
| `--no-histdups` | Allow duplicates in history list  (press RETURN) | no |
| `--mouse` | Ignore mouse input  (press RETURN) | no |
| `--emouse` | Ignore mouse input  (press RETURN) | no |
| `--rmouse` | Normal mouse scroll direction  (press RETURN) | no |
| `--wheel-lines` | Scroll 1 line(s) on mouse wheel  (press RETURN) | no |
| `--save-marks` | Don't save marks in history file  (press RETURN) | no |
| `--line-num-width` | Line number width is 7 chars  (press RETURN) | no |
| `--status-col-width` | Status column width is 2 chars  (press RETURN) | no |
| `--incsearch` | Incremental search is off  (press RETURN) | no |
| `--use-color` | Don't use color  (press RETURN) | no |
| `--file-size` | Don't get size of each file  (press RETURN) | no |
| `--time-file-size` | Time limit for --file-size is 0 milliseconds  (press RETURN) | no |
| `--status-line` | Line highlight applies to text only  (press RETURN) | no |
| `--header` | Header (lines,columns,line-number) is 0,0,0  (press RETURN) | no |
| `--no-paste` | Ignore pasted input  (press RETURN) | yes |
| `--form-feed` | Don't stop on form feed  (press RETURN) | no |
| `--past-eof` | Stop scrolling at end of file  (press RETURN) | no |
| `--no-edit-warn` | Warn when editing a file opened via LESSOPEN  (press RETURN) | no |
| `--no-number-headers` | Number header lines  (press RETURN) | no |
| `--no-search-headers` | Search includes header lines and columns  (press RETURN) | no |
| `--no-search-header-lines` | Search includes header lines and columns  (press RETURN) | no |
| `--no-search-header-columns` | Search includes header lines and columns  (press RETURN) | no |
| `--redraw-on-quit` | Don't redraw screen when quitting  (press RETURN) | no |
| `--search-options` | search options: -  (press RETURN) | no |
| `--exit-follow-on-close` | Don't exit F command when input closes  (press RETURN) | no |
| `--no-vbell` | Display visual bell  (press RETURN) | no |
| `--modelines` | Read 0 lines looking for modelines  (press RETURN) | no |
| `--intr` | interrupt character is ^  (press RETURN) | no |
| `--wordwrap` | Wrap lines at any character  (press RETURN) | no |
| `--show-preproc-errors` | Don't show error message if preprocessor fails  (press RETURN) | no |
| `--proc-backspace` | Backspace handling is specified by the -U option  (press RETURN) | no |
| `--proc-tab` | Tab handling is specified by the -U option  (press RETURN) | no |
| `--proc-return` | Carriage return handling is specified by the -U option  (press RETURN) | no |
| `--match-shift` | Search match shift is .5 of screen width  (press RETURN) | no |
| `--autosave` | Autosave actions: -  (press RETURN) | no |
| `--end-prompt` | Print after short prompt: (nothing)  (press RETURN) | no |
