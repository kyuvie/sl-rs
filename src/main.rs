use std::ffi::*;
use std::env;
use std::sync::Mutex;
use std::time::Duration;
use std::{thread, time};

use pancurses::*;

use once_cell::sync::Lazy;

const D51HEIGHT   : usize =     10usize;
const D51FUNNEL   : usize =      7usize;
const D51LENGTH   : usize =     83usize;
const D51PATTERNS : usize =      6usize;


const D51STR1: &str =  "      ====        ________                ___________ "  ;
const D51STR2: &str =  "  _D _|  |_______/        \\__I_I_____===__|_________| " ;
const D51STR3: &str =  "   |(_)---  |   H\\________/ |   |        =|___ ___|   " ;
const D51STR4: &str =  "   /     |  |   H  |  |     |   |         ||_| |_||   "  ;
const D51STR5: &str =  "  |      |  |   H  |__--------------------| [___] |   "  ;
const D51STR6: &str =  "  | ________|___H__/__|_____/[][]~\\_______|       |   " ;
const D51STR7: &str =  "  |/ |   |-----------I_____I [][] []  D   |=======|__ "  ;

const D51WHL11: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ " ;
const D51WHL12: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        "    ;
const D51WHL13: &str = "  \\_/      \\O=====O=====O=====O_/      \\_/            "  ;

const D51WHL21: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ "  ;
const D51WHL22: &str = " |/-=|___|=O=====O=====O=====O   |_____/~\\___/        "     ;
const D51WHL23: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            ";

const D51WHL31: &str = "__/ =| o |=-O=====O=====O=====O \\ ____Y___________|__ "      ;
const D51WHL32: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        "      ;
const D51WHL33: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            " ;

const D51WHL41: &str = "__/ =| o |=-~O=====O=====O=====O\\ ____Y___________|__ "       ;
const D51WHL42: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        "       ;
const D51WHL43: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            "  ;

const D51WHL51: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ "   ;
const D51WHL52: &str = " |/-=|___|=   O=====O=====O=====O|_____/~\\___/        "      ;
const D51WHL53: &str = "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/            " ;

const D51WHL61: &str = "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|__ " ;
const D51WHL62: &str = " |/-=|___|=    ||    ||    ||    |_____/~\\___/        "    ;
const D51WHL63: &str = "  \\_/      \\_O=====O=====O=====O/      \\_/            "  ;

const D51DEL: &str =   "                                                      " ;

const COAL01: &str = "                              "     ;
const COAL02: &str = "                              "     ;
const COAL03: &str = "    _________________         "     ;
const COAL04: &str = "   _|                \\_____A  "    ;
const COAL05: &str = " =|                        |  "     ;
const COAL06: &str = " -|                        |  "     ;
const COAL07: &str = "__|________________________|_ "     ;
const COAL08: &str = "|__________________________|_ "     ;
const COAL09: &str = "   |_D__D__D_|  |_D__D__D_|   "     ;
const COAL10: &str = "    \\_/   \\_/    \\_/   \\_/    " ;

const COALDEL: &str = "                              " ;

const LOGOHEIGHT   : usize =  	 6usize;
const LOGOFUNNEL   : usize =	 4usize;
const LOGOLENGTH   : usize =    84usize;
const LOGOPATTERNS : usize =	 6usize;

const LOGO1: &str =  "     ++      +------ " ;
const LOGO2: &str =  "     ||      |+-+ |  " ;
const LOGO3: &str =  "   /---------|| | |  " ;
const LOGO4: &str =  "  + ========  +-+ |  " ;

const LWHL11: &str = " _|--O========O~\\-+  "  ;
const LWHL12: &str = "//// \\_/      \\_/    " ;

const LWHL21: &str = " _|--/O========O\\-+  "  ;
const LWHL22: &str = "//// \\_/      \\_/    " ;

const LWHL31: &str = " _|--/~O========O-+  "   ;
const LWHL32: &str = "//// \\_/      \\_/    " ;

const LWHL41: &str = " _|--/~\\------/~\\-+  " ;
const LWHL42: &str = "//// \\_O========O    "  ;

const LWHL51: &str = " _|--/~\\------/~\\-+  " ;
const LWHL52: &str = "//// \\O========O/    "  ;

const LWHL61: &str = " _|--/~\\------/~\\-+  " ;
const LWHL62: &str = "//// O========O_/    "   ;

const LCOAL1: &str = "____                 " ;
const LCOAL2: &str = "|   \\@@@@@@@@@@@     ";
const LCOAL3: &str = "|    \\@@@@@@@@@@@@@_ ";
const LCOAL4: &str = "|                  | " ;
const LCOAL5: &str = "|__________________| " ;
const LCOAL6: &str = "   (O)       (O)     " ;

const LCAR1: &str = "____________________ " ;
const LCAR2: &str = "|  ___ ___ ___ ___ | " ;
const LCAR3: &str = "|  |_| |_| |_| |_| | " ;
const LCAR4: &str = "|__________________| " ;
const LCAR5: &str = "|__________________| " ;
const LCAR6: &str = "   (O)        (O)    " ;

const DELLN: &str = "                     " ;

const C51HEIGHT  : usize =   11usize;
const C51FUNNEL  : usize =    7usize;
const C51LENGTH  : usize =   87usize;
const C51PATTERNS: usize =    6usize;

const C51DEL: &str = "                                                       ";

const C51STR1: &str = "        ___                                            " ;
const C51STR2: &str = "       _|_|_  _     __       __             ___________" ;
const C51STR3: &str = "    D__/   \\_(_)___|  |__H__|  |_____I_Ii_()|_________|";
const C51STR4: &str = "     | `---'   |:: `--'  H  `--'         |  |___ ___|  ";
const C51STR5: &str = "    +|~~~~~~~~++::~~~~~~~H~~+=====+~~~~~~|~~||_| |_||  ";
const C51STR6: &str = "    ||        | ::       H  +=====+      |  |::  ...|  ";
const C51STR7: &str = "|    | _______|_::-----------------[][]-----|       |  ";

const C51WH61: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__"      ;
const C51WH62: &str = "------'|oOo|==[]=-     ||      ||      |  ||=======_|__"       ;
const C51WH63: &str = "/~\\____|___|/~\\_|   O=======O=======O  |__|+-/~\\_|     "    ;
const C51WH64: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       " ;

const C51WH51: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__"      ;
const C51WH52: &str = "------'|oOo|===[]=-    ||      ||      |  ||=======_|__"       ;
const C51WH53: &str = "/~\\____|___|/~\\_|    O=======O=======O |__|+-/~\\_|     "    ;
const C51WH54: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       " ;

const C51WH41: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__"      ;
const C51WH42: &str = "------'|oOo|===[]=- O=======O=======O  |  ||=======_|__"       ;
const C51WH43: &str = "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     "    ;
const C51WH44: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       " ;

const C51WH31: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__"      ;
const C51WH32: &str = "------'|oOo|==[]=- O=======O=======O   |  ||=======_|__"       ;
const C51WH33: &str = "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     "    ;
const C51WH34: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       " ;

const C51WH21: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__"      ;
const C51WH22: &str = "------'|oOo|=[]=- O=======O=======O    |  ||=======_|__"       ;
const C51WH23: &str = "/~\\____|___|/~\\_|      ||      ||      |__|+-/~\\_|     "    ;
const C51WH24: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       " ;

const C51WH11: &str = "| /~~ ||   |-----/~~~~\\  /[I_____I][][] --|||_______|__"      ;
const C51WH12: &str = "------'|oOo|=[]=-      ||      ||      |  ||=======_|__"       ;
const C51WH13: &str = "/~\\____|___|/~\\_|  O=======O=======O   |__|+-/~\\_|     "    ;
const C51WH14: &str = "\\_/         \\_/  \\____/  \\____/  \\____/      \\_/       " ;


static ACCIDENT : Lazy<Mutex<i32>> = Lazy::new(||Mutex::new(0i32));
static LOGO     : Lazy<Mutex<i32>> = Lazy::new(||Mutex::new(0i32));
static FLY      : Lazy<Mutex<i32>> = Lazy::new(||Mutex::new(0i32));
static C51      : Lazy<Mutex<i32>> = Lazy::new(||Mutex::new(0i32));


fn my_mvaddstr(win: &Window, y: c_int, x: c_int, str: &str) -> c_int
{
    let mut x = x;
    let mut ind_str = 0usize;
    while x < 0 {
        if ind_str >= str.chars().count() {
            return ERR;
        }
        x += 1;
        ind_str += 1;
    }

    while ind_str < str.chars().count() {
        if win.mvaddch(y, x, str.chars().nth(ind_str).unwrap()) == ERR {
            return ERR;
        }
        ind_str += 1;
        x += 1;
    }

    return OK;
}

fn option(str: &str)
{
    let ind_str = 0usize;

    while ind_str < str.chars().count() {
        match str.chars().nth(ind_str).unwrap() {
            'a' => {
                *ACCIDENT.lock().unwrap() = 1i32;
                break;
            },
            'F' => {
                *FLY.lock().unwrap()      = 1i32;
                break;
            },
            'l' => {
                *LOGO.lock().unwrap()     = 1i32;
                break;
            },
            'c' => {
                *C51.lock().unwrap()      = 1i32;
                break;
            },
            _ => {
                break;
            }
        }
    }
}

const FOURTY_MILLIS: Duration = time::Duration::from_millis(40);


fn main()
{

    let args: Vec<String> = env::args().collect();

    for i in 1..args.len() {
        if args[i].chars().nth(0).unwrap() == '-' {
            option(&args[i][1..]);
        }
    }

    let stdscr = initscr();
    // signal(SIGINT, SIG_IGN);
    noecho();
    curs_set(0);
    stdscr.nodelay(true);
    // stdscr.leaveok(true);
    stdscr.scrollok(false);

    let mut x = stdscr.get_max_x() - 1;

    loop {
        if *LOGO.lock().unwrap() == 1 {
            if add_sl(&stdscr, x) == ERR {
                break;
            }
        }
        else if *C51.lock().unwrap() == 1 {
            if add_C51(&stdscr, x) == ERR {
                break;
            }
        }
        else {
            if add_D51(&stdscr, x) == ERR {
                break;
            }
        }
        stdscr.getch();
        stdscr.refresh();
        thread::sleep(FOURTY_MILLIS);
        x -= 1;
    }
    stdscr.mv( stdscr.get_max_y() - 1, 0); // mvcur?
    endwin();

}


fn add_sl(win: &Window, x:i32) -> i32
{

    let sl: [[&str; LOGOHEIGHT + 1]; LOGOPATTERNS]
        = [[LOGO1, LOGO2, LOGO3, LOGO4, LWHL11, LWHL12, DELLN],
           [LOGO1, LOGO2, LOGO3, LOGO4, LWHL21, LWHL22, DELLN],
           [LOGO1, LOGO2, LOGO3, LOGO4, LWHL31, LWHL32, DELLN],
           [LOGO1, LOGO2, LOGO3, LOGO4, LWHL41, LWHL42, DELLN],
           [LOGO1, LOGO2, LOGO3, LOGO4, LWHL51, LWHL52, DELLN],
           [LOGO1, LOGO2, LOGO3, LOGO4, LWHL61, LWHL62, DELLN]];

    let coal: [&str; LOGOHEIGHT + 1]
        = [LCOAL1, LCOAL2, LCOAL3, LCOAL4, LCOAL5, LCOAL6, DELLN];

    let car: [&str; LOGOHEIGHT + 1]
        = [LCAR1, LCAR2, LCAR3, LCAR4, LCAR5, LCAR6, DELLN];

    #[allow(unused_assignments)]
    let (mut y, mut py1, mut py2, mut py3) = (0, 0, 0, 0);

    if x < - (LOGOLENGTH as i32) {
          return ERR;
    }
    y = win.get_max_y() / 2 - 3;

    if *FLY.lock().unwrap() == 1 {
        y = (x / 6) + win.get_max_y() - (win.get_max_x() / 6) - LOGOHEIGHT as i32;
        py1 = 2;  py2 = 4;  py3 = 6;
    }
    for i in 0..=LOGOHEIGHT as i32 {
        my_mvaddstr(win, y + i, x, sl[(LOGOLENGTH as i32 + x) as usize / 3 % LOGOPATTERNS][i as usize]);
        my_mvaddstr(win, y + i + py1, x + 21, coal[i as usize]);
        my_mvaddstr(win, y + i + py2, x + 42, car[i as usize]);
        my_mvaddstr(win, y + i + py3, x + 63, car[i as usize]);
    }
    if *ACCIDENT.lock().unwrap() == 1 {
        add_man(win, y + 1, x + 14);
        add_man(win, y + 1 + py2, x + 45);  add_man(win, y + 1 + py2, x + 53);
        add_man(win, y + 1 + py3, x + 66);  add_man(win, y + 1 + py3, x + 74);
    }
    add_smoke(win, y - 1, x + LOGOFUNNEL as i32);
    return OK;

}

#[allow(non_snake_case)]
fn add_D51(win: &Window, x: i32) -> i32
{
    let d51: [[&str; D51HEIGHT + 1];D51PATTERNS]
        = [[D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7,
            D51WHL11, D51WHL12, D51WHL13, D51DEL],
           [D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7,
            D51WHL21, D51WHL22, D51WHL23, D51DEL],
           [D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7,
            D51WHL31, D51WHL32, D51WHL33, D51DEL],
           [D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7,
            D51WHL41, D51WHL42, D51WHL43, D51DEL],
           [D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7,
            D51WHL51, D51WHL52, D51WHL53, D51DEL],
           [D51STR1, D51STR2, D51STR3, D51STR4, D51STR5, D51STR6, D51STR7,
            D51WHL61, D51WHL62, D51WHL63, D51DEL]];
    let coal: [&str; D51HEIGHT + 1]
        = [COAL01, COAL02, COAL03, COAL04, COAL05,
           COAL06, COAL07, COAL08, COAL09, COAL10, COALDEL];

    #[allow(unused_assignments)]
    let mut y = 0;
    let mut l_dy = 0;

    if x < - (D51LENGTH as i32){
        return ERR;
    }
    y = win.get_max_y() / 2 - 5;

    if *FLY.lock().unwrap() == 1 {
        y = (x / 7) + win.get_max_y() - (win.get_max_x() / 7) - D51HEIGHT as i32;
        l_dy = 1;
    }
    for i in 0..= D51HEIGHT {
        my_mvaddstr(win, y + i as i32, x, d51[(D51LENGTH as i32 + x) as usize % D51PATTERNS][i]);
        my_mvaddstr(win, y + i as i32 + l_dy, x + 53, coal[i]);
    }
    if *ACCIDENT.lock().unwrap() == 1 {
        add_man(win, y + 2, x + 43);
        add_man(win, y + 2, x + 47);
    }
    add_smoke(win, y - 1, x + D51FUNNEL as i32);
    return OK;
}

#[allow(non_snake_case)]
fn add_C51(win: &Window, x: i32) -> i32
{

    let c51: [[&str; C51HEIGHT + 1]; C51PATTERNS]
        = [[C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7,
            C51WH11, C51WH12, C51WH13, C51WH14, C51DEL],
           [C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7,
            C51WH21, C51WH22, C51WH23, C51WH24, C51DEL],
           [C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7,
            C51WH31, C51WH32, C51WH33, C51WH34, C51DEL],
           [C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7,
            C51WH41, C51WH42, C51WH43, C51WH44, C51DEL],
           [C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7,
            C51WH51, C51WH52, C51WH53, C51WH54, C51DEL],
           [C51STR1, C51STR2, C51STR3, C51STR4, C51STR5, C51STR6, C51STR7,
            C51WH61, C51WH62, C51WH63, C51WH64, C51DEL]];
    let coal: [&str; C51HEIGHT + 1]
        = [COALDEL, COAL01, COAL02, COAL03, COAL04, COAL05,
           COAL06, COAL07, COAL08, COAL09, COAL10, COALDEL];

    #[allow(unused_assignments)]
    let (mut y, mut dy) = (0, 0);

    if x < - (C51LENGTH as i32) {
        return ERR;
    }
    y = win.get_max_y() / 2 - 5;

    if *FLY.lock().unwrap() == 1 {
        y = (x / 7) + win.get_max_y() - (win.get_max_x() / 7) - C51HEIGHT as i32;
        dy = 1;
    }
    for i in 0..= C51HEIGHT as i32 {
        my_mvaddstr(win, y + i, x, c51[(C51LENGTH as i32 + x) as usize % C51PATTERNS][i as usize]);
        my_mvaddstr(win, y + i + dy, x + 55, coal[i as usize]);
    }
    if *ACCIDENT.lock().unwrap() == 1 {
        add_man(win, y + 3, x + 45);
        add_man(win, y + 3, x + 49);
    }
    add_smoke(win, y - 1, x + C51FUNNEL as i32);
    return OK;

}


fn add_man(win: &Window, y: i32, x: i32)
{
    let man: [[&str; 2]; 2] = [["", "(O)"], ["Help!", "\\O/"]];

    for i in 0..2 {
        my_mvaddstr(win, y + i, x, man[(LOGOLENGTH as i32 + x) as usize / 12usize % 2usize][i as usize]);
    }
}


const SMOKEPTNS: usize       = 16usize;


fn add_smoke(win: &Window, y: i32, x: i32)
{
    #[allow(non_camel_case_types)]
    #[derive(Clone, Copy, Default)]
    struct smokes {
        y: i32,
        x: i32,
        ptrn: usize,
        kind: usize,
    }

    #[allow(non_upper_case_globals)]
    static mut sum: usize = 0usize;

    #[allow(non_upper_case_globals)]
    static Smoke: [[&str; SMOKEPTNS]; 2]
    = [["(   )", "(    )", "(    )", "(   )", "(  )",
        "(  )" , "( )"   , "( )"   , "()"   , "()"  ,
        "O"    , "O"     , "O"     , "O"    , "O"   ,
        " "                                          ],
       ["(@@@)", "(@@@@)", "(@@@@)", "(@@@)", "(@@)",
        "(@@)" , "(@)"   , "(@)"   , "@@"   , "@@"  ,
        "@"    , "@"     , "@"     , "@"    , "@"   ,
        " "                                          ]];

    #[allow(non_upper_case_globals)]
    static Eraser: [&str; SMOKEPTNS]
    =  ["     ", "      ", "      ", "     ", "    ",
        "    " , "   "   , "   "   , "  "   , "  "  ,
        " "    , " "     , " "     , " "    , " "   ,
        " "                                          ];

    #[allow(non_upper_case_globals)]
    static dy: [i32; SMOKEPTNS] = [ 2,  1, 1, 1, 0, 0, 0, 0, 0, 0,
            0,  0, 0, 0, 0, 0             ];

    #[allow(non_upper_case_globals)]
    static dx: [i32; SMOKEPTNS] = [-2, -1, 0, 1, 1, 1, 1, 1, 2, 2,
            2,  2, 2, 3, 3, 3             ];

    #[allow(non_snake_case)]
    static mut S: Lazy<[smokes; 1000]> = Lazy::new(||[smokes::default(); 1000]);

    if x % 4 == 0 {
        for i in 0..unsafe { sum } {
            my_mvaddstr(win, unsafe { S[i].y }, unsafe { S[i].x }, Eraser[unsafe { S[i].ptrn }]);
            unsafe { S[i].y    -= dy[S[i].ptrn] };
            unsafe { S[i].x    += dx[S[i].ptrn] };
            unsafe { S[i].ptrn += if S[i].ptrn < SMOKEPTNS - 1 { 1 } else { 0 } };
            my_mvaddstr(win, unsafe { S[i].y} , unsafe { S[i].x } , Smoke[unsafe { S[i].kind }][unsafe { S[i].ptrn }]);
        }
        my_mvaddstr(win, y, x, Smoke[ unsafe { sum } % 2][0]);
        unsafe { S[sum].y = y };    unsafe { S[sum].x = x };
        unsafe { S[sum].ptrn = 0 }; unsafe { S[sum].kind = sum % 2 };
        unsafe { sum += 1; }
    }
}
