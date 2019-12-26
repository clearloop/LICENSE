use license::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let len = args.len();

    match len {
        1 => helper(),
        _ => match args[1].as_str() {
            "list" => list(),
            "help" => helper(),
            // Licenses
            "afl-3.0" => w(afl::AFL_V3),
            "apache-2.0" => w(apache::APACHE_V2),
            "artistic-2.0" => w(artistic::ARTISTIC_V2),
            "bsl-1.0" => w(bsl::BSL_V1),
            "bsd-2-clause" => w(bsd::BSD_V2),
            "bsd-3-clause" => w(bsd::BSD_V3),
            "bsd-3-clause-clear" => w(bsd::BSD_V3_CLEAR),
            "cc0-1.0" => w(cc::CC_V1),
            "cc-by-4.0" => w(cc::CC_BY_V4),
            "cc-by-sa-4.0" => w(cc::CC_BY_SA_V4),
            "wtfpl" => w(wtfpl::WTFPL),
            "ecl-2.0" => w(ecl::ECL_V2),
            "epl-1.0" => w(epl::EPL_V1),
            "eupl-1.1" => w(eupl::EUPL_V1_1),
            "agpl-3.0" => w(agpl::AGPL_V3),
            "gpl-2.0" => w(gpl::GPL_V2_1),
            "gpl-3.0" => w(gpl::GPL_V3),
            "lgpl-2.1" => w(lgpl::LGPL_V2_1),
            "lgpl-3.0" => w(lgpl::LGPL_V3),
            "isc" => w(isc::ISC),
            "lppl-1.3c" => w(lppl::LPPL_V1_3),
            "ms-pl" => w(ms_pl::MS_PL),
            "mit" => w(mit::MIT),
            "mpl-2.0" => w(mpl::MPL_V2),
            "osl-3.0" => w(osl::OSL_V3),
            "postgresql" => w(psql::PSQL),
            "ofl-1.1" => w(ofl::OFL_V1_1),
            "ncsa" => w(ncsa::NCSA),
            "unlicense" => w(unlicense::UNLICENSE),
            "zlib" => w(zlib::ZLIB),
            // info
            "info" => match args[2].as_str() {
                "afl-3.0" => println!("{}", afl::AFL_V3),
                "apache-2.0" => println!("{}", apache::APACHE_V2),
                "artistic-2.0" => println!("{}", artistic::ARTISTIC_V2),
                "bsl-1.0" => println!("{}", bsl::BSL_V1),
                "bsd-2-clause" => println!("{}", bsd::BSD_V2),
                "bsd-3-clause" => println!("{}", bsd::BSD_V3),
                "bsd-3-clause-clear" => println!("{}", bsd::BSD_V3_CLEAR),
                "cc0-1.0" => println!("{}", cc::CC_V1),
                "cc-by-4.0" => println!("{}", cc::CC_BY_V4),
                "cc-by-sa-4.0" => println!("{}", cc::CC_BY_SA_V4),
                "wtfpl" => println!("{}", wtfpl::WTFPL),
                "ecl-2.0" => println!("{}", ecl::ECL_V2),
                "epl-1.0" => println!("{}", epl::EPL_V1),
                "eupl-1.1" => println!("{}", eupl::EUPL_V1_1),
                "agpl-3.0" => println!("{}", agpl::AGPL_V3),
                "gpl-2.0" => println!("{}", gpl::GPL_V2_1),
                "gpl-3.0" => println!("{}", gpl::GPL_V3),
                "lgpl-2.1" => println!("{}", lgpl::LGPL_V2_1),
                "lgpl-3.0" => println!("{}", lgpl::LGPL_V3),
                "isc" => println!("{}", isc::ISC),
                "lppl-1.3c" => println!("{}", lppl::LPPL_V1_3),
                "ms-pl" => println!("{}", ms_pl::MS_PL),
                "mit" => println!("{}", mit::MIT),
                "mpl-2.0" => println!("{}", mpl::MPL_V2),
                "osl-3.0" => println!("{}", osl::OSL_V3),
                "postgresql" => println!("{}", psql::PSQL),
                "ofl-1.1" => println!("{}", ofl::OFL_V1_1),
                "ncsa" => println!("{}", ncsa::NCSA),
                "unlicense" => println!("{}", unlicense::UNLICENSE),
                "zlib" => println!("{}", zlib::ZLIB),
                _ => list()
            },
            // Empty input
            _ => list()
        }
    }
}

fn w(l: &'static str) {
    std::fs::write("LICENSE", &l[1..]).unwrap();
    println!("ok!");
}

fn list() {
    println!(r#"Supports Licenses:
afl-3.0                            Academic Free License v3.0
apache-2.0                         Apache license 2.0
artistic-2.0                       Artistic license 2.0
bsl-1.0                            Boost Software License 1.0
bsd-2-clause                       BSD 2-clause "Simplified" license
bsd-3-clause                       BSD 3-clause "New" or "Revised" license
bsd-3-clause-clear                 BSD 3-clause Clear license
cc                                 Creative Commons license family	
cc0-1.0                            Creative Commons Zero v1.0 Universal
cc-by-4.0                          Creative Commons Attribution 4.0
cc-by-sa-4.0                       Creative Commons Attribution Share Alike 4.0
wtfpl                              Do What The F*ck You Want To Public License
ecl-2.0                            Educational Community License v2.0
epl-1.0                            Eclipse Public License 1.0
eupl-1.1                           European Union Public License 1.1	
agpl-3.0                           GNU Affero General Public License v3.0
gpl                                GNU General Public License family	
gpl-2.0                            GNU General Public License v2.0	
gpl-3.0                            GNU General Public License v3.0	
lgpl                               GNU Lesser General Public License family
lgpl-2.1                           GNU Lesser General Public License v2.1
lgpl-3.0                           GNU Lesser General Public License v3.0
isc                                ISC
lppl-1.3c                          LaTeX Project Public License v1.3c
ms-pl                              Microsoft Public License
mit                                MIT
mpl-2.0                            Mozilla Public License 2.0
osl-3.0                            Open Software License 3.0
postgresql                         PostgreSQL License
ofl-1.1                            SIL Open Font License 1.1	
ncsa                               University of Illinois/NCSA Open Source License
unlicense                          The Unlicense
zlib                               zLib License

Example: 
license mit ———— Generate a MIT LICENSE"#)
}

fn helper() {
    println!(r#"clearloop <udtrokia@163.com>

Usage:
license <name>                     Generate a LICENSE
license list                       Show available LICENSEs' list
license info <name>                Print license's content in command-line"#);
}
