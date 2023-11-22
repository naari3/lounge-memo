use std::{collections::HashMap, fmt::Display};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use strsim::levenshtein;

use crate::word::{normalize_japanese_characters, Word};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Console {
    SFC,
    GBA,
    N64,
    GC,
    DS,
    Wii,
    _3DS,
    New,
    Tour,
}

impl Display for Console {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Console::SFC => write!(f, "SFC"),
            Console::GBA => write!(f, "GBA"),
            Console::N64 => write!(f, "N64"),
            Console::GC => write!(f, "GC"),
            Console::DS => write!(f, "DS"),
            Console::Wii => write!(f, "Wii"),
            Console::_3DS => write!(f, "3DS"),
            Console::New => write!(f, ""),
            Console::Tour => write!(f, "Tour"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Course {
    name: String,
    console: Console,
}

impl Course {
    pub fn new(name: String, console: Console) -> Self {
        Self { name, console }
    }
}

impl Display for Course {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // もしConsoleがNewだったら、シリーズ名を表示しない
        // それ以外の場合はスペースを間に挟んでシリーズ名を表示する
        if self.console == Console::New {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{} {}", self.console, self.name)
        }
    }
}

pub static COURSE_SHORTHAND_MAP: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
    let map = HashMap::new();
    // COURSESの組み立て時に埋める
    Mutex::new(map)
});

pub static COURSES: Lazy<Mutex<Vec<Course>>> = Lazy::new(|| {
    let mut courses = Vec::new();
    for (name, console, shorthands) in [
        // https://github.com/sheat-git/mk8dx.py/blob/main/mk8dx/data.py#L311
        // キノコカップ
        (
            "マリオカートスタジアム",
            Console::New,
            vec!["mks", "ﾏﾘｵｶｰﾄｽﾀｼﾞｱﾑ", "ﾏﾘｶｽ"],
        ),
        (
            "ウォーターパーク",
            Console::New,
            vec!["wp", "ｳｫｰﾀﾊﾟｰｸ", "ｦｰﾀｰﾊﾟｰｸ", "ｳｫﾀﾊﾟ", "ｦﾀﾊﾟ", "ｵﾀﾊﾟ"],
        ),
        (
            "スイーツキャニオン",
            Console::New,
            vec!["ssc", "ｽｲｰﾂｷｬﾆｵﾝ", "ｽｲｷｬﾆ"],
        ),
        (
            "ドッスンいせき",
            Console::New,
            vec!["tr", "ﾄﾞｯｽﾝｲｾｷ", "ﾄﾞｯｽﾝ", "ｲｾｷ", "ﾄﾞｯｽﾝ遺跡", "遺跡"],
        ),
        // フラワーカップ
        (
            "マリオサーキット",
            Console::New,
            vec!["mc", "ﾏﾘｵｻｰｷｯﾄ", "ﾏﾘｻ", "新ﾏﾘｻ", "ｼﾝﾏﾘｻ"],
        ),
        (
            "キノピオハーバー",
            Console::New,
            vec!["th", "ｷﾉﾋﾟｵﾊｰﾊﾞｰ", "ﾊｰﾊﾞｰ"],
        ),
        (
            "ねじれマンション",
            Console::New,
            vec![
                "tm",
                "ﾈｼﾞﾚﾏﾝｼｮﾝ",
                "ﾈｼﾞﾏﾝ",
                "ﾈｼﾞﾚ",
                "ﾈｼﾞｼｮﾝ",
                "ﾈｼﾞ",
                "ﾈｼﾞﾈｼﾞ",
                "ﾏﾝｼｮﾝ",
            ],
        ),
        (
            "ヘイホーこうざん",
            Console::New,
            vec!["sgf", "ﾍｲﾎｰｺｳｻﾞﾝ", "ﾍｲﾎｰ鉱山", "ﾍｲｺｰ", "ﾍｲｺｳ", "ﾍｲ鉱"],
        ),
        // スターカップ
        (
            "サンシャインくうこう",
            Console::New,
            vec!["sa", "ｻﾝｼｬｲﾝｸｳｺｳ", "空港", "ｸｳｺｳ", "ｻﾝｼｬｲﾝ"],
        ),
        (
            "ドルフィンみさき",
            Console::New,
            vec!["ds", "ﾄﾞﾙﾌｨﾝﾐｻｷ", "ﾄﾞﾙﾐ", "ﾐｻｷ", "ﾄﾞﾙﾌｨﾝ岬", "岬"],
        ),
        (
            "エレクトロドリーム",
            Console::New,
            vec!["ed", "ｴﾚｸﾄﾛﾄﾞﾘｰﾑ", "ｴﾚﾄﾞ", "ｴﾚﾄﾞﾘ"],
        ),
        (
            "ワリオスノーマウンテン",
            Console::New,
            vec![
                "mw",
                "ﾜﾘｵｽﾉｰﾏｳﾝﾃﾝ",
                "ﾜﾘｽﾉ",
                "ﾕｷﾔﾏ",
                "雪山",
                "ｽﾉ",
                "ﾕｷﾔﾏｳﾝﾃﾝ",
            ],
        ),
        // スペシャルカップ
        (
            "スカイガーデン",
            Console::New,
            vec!["cc", "ｽｶｲｶﾞｰﾃﾞﾝ", "ｽｶｶﾞ"],
        ),
        (
            "ホネホネさばく",
            Console::New,
            vec!["bdd", "ﾎﾈﾎﾈｻﾊﾞｸ", "ﾎﾈｻﾊﾞ", "ﾎﾈﾎﾈ"],
        ),
        (
            "クッパキャッスル",
            Console::New,
            vec!["bc", "ｸｯﾊﾟｷｬｯｽﾙ", "ｸﾊﾟｷｬ", "ｸｷｬﾊﾟ", "ｸｯｷｬﾊﾟｯｽﾙ"],
        ),
        (
            "レインボーロード",
            Console::New,
            vec!["rr", "ﾚｲﾝﾎﾞｰﾛｰﾄﾞ", "新虹", "ｼﾝﾆｼﾞ"],
        ),
        // たまごカップ
        (
            "ヨッシーサーキット",
            Console::GC,
            vec!["dyc", "yc", "ﾖｯｼｰｻｰｷｯﾄ", "ﾖｼｻ"],
        ),
        (
            "エキサイトバイク",
            Console::New,
            vec!["dea", "ea", "ｴｷｻｲﾄﾊﾞｲｸ", "ｴｷﾊﾞ"],
        ),
        (
            "ドラゴンロード",
            Console::New,
            vec!["ddd", "dd", "ﾄﾞﾗｺﾞﾝﾛｰﾄﾞ", "ﾄﾞﾗﾛ"],
        ),
        (
            "ミュートシティ",
            Console::New,
            vec!["dmc", "ﾐｭｰﾄｼﾃｨ", "ﾐｭｰﾄ"],
        ),
        // どうぶつカップ
        (
            "ベビィパーク",
            Console::GC,
            vec!["dbp", "bp", "ﾍﾞﾋﾞｨﾊﾟｰｸ", "ﾍﾞﾋﾞｰﾊﾟｰｸ", "ﾍﾞﾋﾞﾊﾟ"],
        ),
        (
            "チーズランド",
            Console::GBA,
            vec!["dcl", "cl", "ﾁｰｽﾞﾗﾝﾄﾞ", "ﾁｰｽﾞ"],
        ),
        (
            "ネイチャーロード",
            Console::New,
            vec!["dww", "ww", "ﾈｲﾁｬｰﾗﾝﾄﾞ", "ﾈｲﾁｬｰ", "ﾅﾁｭﾚ"],
        ),
        (
            "どうぶつの森",
            Console::New,
            vec![
                "dac",
                "ac",
                "ﾄﾞｳﾌﾞﾂﾉﾓﾘ",
                "ﾄﾞｳﾓﾘ",
                "ﾌﾞﾂﾓﾘ",
                "ﾄﾞｳ森",
                "ﾌﾞﾂ森",
                "ﾄﾞｳﾌﾞﾂﾉ森",
            ],
        ),
        // こうらカップ
        (
            "モーモーカントリー",
            Console::Wii,
            vec!["rmmm", "mmm", "ﾓｰﾓｰｶﾝﾄﾘｰ", "ﾓﾓｶﾝ", "ﾓｰﾓｰ"],
        ),
        (
            "マリオサーキット",
            Console::GBA,
            vec!["rmc", "gba", "ｸﾞﾊﾞ", "gbaﾏﾘｵｻｰｷｯﾄ", "gbaﾏﾘｻ"],
        ),
        (
            "プクプクビーチ",
            Console::DS,
            vec!["rccb", "ccb", "ﾌﾟｸﾌﾟｸﾋﾞｰﾁ", "ﾌﾟｸﾌﾟｸ", "ﾌﾟｸﾋﾞ"],
        ),
        (
            "キノピオハイウェイ",
            Console::N64,
            vec!["rtt", "tt", "ｷﾉﾋﾟｵﾊｲｳｪｲ", "ﾊｲｳｪｲ"],
        ),
        // バナナカップ
        (
            "カラカラさばく",
            Console::GC,
            vec![
                "rddd",
                "ｶﾗｶﾗｻﾊﾞｸ",
                "ｶﾗｻﾊﾞ",
                "ｻﾊﾞｸ",
                "gcｶﾗ",
                "gcｶﾗｻﾊﾞ",
                "gcｻﾊﾞ",
            ],
        ),
        (
            "ドーナツへいや3",
            Console::SFC,
            vec!["rdp3", "rdp", "dp3", "ﾄﾞｰﾅﾂﾍｲﾔ", "ﾍｲﾔ", "ﾄﾞｰﾅﾂ平野", "平野"],
        ),
        (
            "ピーチサーキット",
            Console::N64,
            vec!["rrry", "rry", "ﾋﾟｰﾁｻｰｷｯﾄ", "ﾋﾟﾁｻ"],
        ),
        (
            "DKジャングル",
            Console::_3DS,
            vec!["rdkj", "dk", "dkj", "dkｼﾞｬﾝｸﾞﾙ", "ｼﾞｬﾝｸﾞﾙ"],
        ),
        // このはカップ
        (
            "ワリオスタジアム",
            Console::DS,
            vec!["rws", "ws", "ﾜﾘｵｽﾀｼﾞｱﾑ", "ﾜﾘｽﾀ"],
        ),
        (
            "シャーベットランド",
            Console::GC,
            vec!["rsl", "sl", "ｼｬｰﾍﾞｯﾄﾗﾝﾄﾞ", "ｼｬｰﾍﾞｯﾄ", "ｼｬﾍﾞﾗﾝ", "ｼｬﾍﾞ"],
        ),
        (
            "ミュージックパーク",
            Console::_3DS,
            vec!["rmp", "mp", "ﾐｭｰｼﾞｯｸﾊﾟｰｸ", "ﾐｭｰﾊﾟ"],
        ),
        (
            "ヨッシーバレー",
            Console::N64,
            vec!["ryv", "yv", "ﾖｯｼｰﾊﾞﾚｰ", "ﾖｼﾊﾞ"],
        ),
        // サンダーカップ
        (
            "チクタクロック",
            Console::DS,
            vec!["rttc", "ttc", "ﾁｸﾀｸﾛｯｸ", "ﾁｸﾀｸ"],
        ),
        (
            "パックンスライダー",
            Console::_3DS,
            vec!["rpps", "pps", "ﾊﾟｯｸﾝｽﾗｲﾀﾞｰ", "ﾊﾟｸｽﾗ", "ﾊﾟｯｸﾝ"],
        ),
        (
            "グラグラかざん",
            Console::Wii,
            vec!["rgv", "gv", "ｸﾞﾗｸﾞﾗｶｻﾞﾝ", "ｸﾞﾗｸﾞﾗ", "ｶｻﾞﾝ"],
        ),
        (
            "レインボーロード",
            Console::N64,
            vec!["rrrd", "rrd", "64ﾚｲﾝﾎﾞｰﾛｰﾄﾞ", "64ﾆｼﾞ", "64虹", "ﾛｸﾖﾝ"],
        ),
        // ゼルダカップ
        (
            "ワリオこうざん",
            Console::Wii,
            vec!["dwgm", "wgm", "ﾜﾘｵｺｳｻﾞﾝ", "ﾜﾘｺｳ", "ﾜﾘｵ鉱山", "ﾜﾘ鉱"],
        ),
        (
            "レインボーロード",
            Console::SFC,
            vec!["drr", "sfcﾆｼﾞ", "sfcﾚｲﾝﾎﾞｰﾛｰﾄﾞ", "sfc虹", "sfc"],
        ),
        (
            "ツルツルツイスター",
            Console::New,
            vec!["diio", "iio", "ﾂﾙﾂﾙﾂｲｽﾀｰ", "ﾂﾂﾂ", "ﾂﾙﾂﾙ"],
        ),
        (
            "ハイラルサーキット",
            Console::New,
            vec!["dhc", "hc", "ﾊｲﾗﾙｻｰｷｯﾄ", "ﾊｲﾗﾙ"],
        ),
        // ベルカップ
        (
            "ネオクッパシティ",
            Console::_3DS,
            vec!["dnbc", "nbc", "ﾈｵｸｯﾊﾟｼﾃｨ", "ﾈｵﾊﾟ", "ﾈｵｸｯﾊﾟ"],
        ),
        (
            "リボンロード",
            Console::GBA,
            vec!["drir", "rir", "ﾘﾎﾞﾝﾛｰﾄﾞ", "ﾘﾎﾞﾝ"],
        ),
        (
            "リンリンメトロ",
            Console::New,
            vec!["dsbs", "sbs", "ﾘﾝﾘﾝﾒﾄﾛ", "ﾘﾝﾒﾄ"],
        ),
        ("ビッグブルー", Console::New, vec!["dbb", "bb", "ﾋﾞｯｸﾞﾌﾞﾙｰ"]),
        // パワフルカップ
        (
            "パリプロムナード",
            Console::Tour,
            vec!["bpp", "pp", "paris", "ﾊﾟﾘﾌﾟﾛﾑﾅｰﾄﾞ", "ﾊﾟﾘ"],
        ),
        (
            "キノピオサーキット",
            Console::_3DS,
            vec!["btc", "tc", "ｷﾉﾋﾟｵｻｰｷｯﾄ", "ｷﾉｻ"],
        ),
        (
            "チョコマウンテン",
            Console::N64,
            vec![
                "bcmo",
                "bcm64",
                "bchm",
                "cmo",
                "chm",
                "cm64",
                "ﾁｮｺﾏｳﾝﾃﾝ",
                "ﾁｮｺ",
                "ﾁｮｺﾏ",
            ],
        ),
        (
            "ココナッツモール",
            Console::Wii,
            vec![
                "bcma",
                "bcom",
                "bcmw",
                "cma",
                "com",
                "cmw",
                "ｺｺﾅｯﾂﾓｰﾙ",
                "ｺｺﾓ",
                "ｺｺﾅｯﾂ",
                "ﾅｯﾂ",
            ],
        ),
        // まねきネコカップ
        (
            "トーキョースクランブル",
            Console::Tour,
            vec![
                "btb",
                "tb",
                "tokyo",
                "ﾄｰｷｮｰｽｸﾗﾝﾌﾞﾙ",
                "ｽｸﾗﾝﾌﾞﾙ",
                "ﾄｰｷｮｰ",
                "ﾄｳｷｮｳ",
                "ﾄｰｷｮｳ",
                "ﾄｳｷｮｰ",
                "東京",
            ],
        ),
        (
            "キノコリッジウェイ",
            Console::DS,
            vec![
                "bsr",
                "sr",
                "ｷﾉｺﾘｯｼﾞｳｪｲ",
                "ｷﾉｺﾘｯｼﾞ",
                "ﾘｯｼﾞｳｪｲ",
                "ｷﾉｺﾘ",
                "ｷｺﾘ",
                "ﾘｯｼﾞ",
            ],
        ),
        (
            "スカイガーデン",
            Console::GBA,
            vec![
                "bsg",
                "sg",
                "gbaｽｶｲｶﾞｰﾃﾞﾝ",
                "gbaｽｶ",
                "ｸﾞﾊﾞｽｶ",
                "ｸﾞﾊﾞｽｶｶﾞ",
                "gbaｽｶｶﾞ",
            ],
        ),
        (
            "ニンニンドージョー",
            Console::New,
            vec!["bnh", "nh", "ﾆﾝﾆﾝﾄﾞｰｼﾞｮｰ", "ﾆﾝｼﾞｮｰ", "ﾆﾝﾆﾝ"],
        ),
        // カブカップ
        (
            "ニューヨークドリーム",
            Console::Tour,
            vec!["bnym", "nym", "ﾆｭｰﾖｰｸﾄﾞﾘｰﾑ", "ﾆｭｰﾖｰｸ", "ﾆｭｰﾄﾞﾘ", "ny"],
        ),
        (
            "マリオサーキット3",
            Console::SFC,
            vec![
                "bmc3",
                "mc3",
                "ﾏﾘｵｻｰｷｯﾄ3",
                "ﾏﾘｻ3",
                "sfcﾏﾘｻ",
                "sfcﾏﾘｵｻｰｷｯﾄ",
                "sfcﾏﾘｻ3",
                "sfcﾏﾘｵｻｰｷｯﾄ3",
            ],
        ),
        (
            "カラカラさばく",
            Console::N64,
            vec!["bkd", "kd", "64ｶﾗｻﾊﾞ", "64ｶﾗ", "64ｻﾊﾞ"],
        ),
        (
            "ワルイージピンボール",
            Console::DS,
            vec!["bwp", "ﾜﾙｲｰｼﾞﾋﾟﾝﾎﾞｰﾙ", "ﾜﾙﾋﾟﾝ", "ﾋﾟﾝﾎﾞｰﾙ"],
        ),
        // プロペラカップ
        (
            "シドニーサンシャイン",
            Console::Tour,
            vec!["bss", "ss", "bsys", "sys", "ｼﾄﾞﾆｰｻﾝｼｬｲﾝ", "ｼﾄﾞﾆｰ"],
        ),
        ("スノーランド", Console::GBA, vec!["bsl", "ｽﾉｰﾗﾝﾄﾞ", "ｽﾉﾗﾝ"]),
        (
            "キノコキャニオン",
            Console::Wii,
            vec!["bmg", "mg", "ｷﾉｺｷｬﾆｵﾝ", "ｷﾉｷｬﾆ", "ｷｬﾆｵﾝ"],
        ),
        (
            "アイスビルディング",
            Console::New,
            vec!["bshs", "shs", "ｱｲｽﾋﾞﾙﾃﾞｨﾝｸﾞ", "ｱｲｽ"],
        ),
        // ゴロいわカップ
        (
            "ロンドンアベニュー",
            Console::Tour,
            vec!["bll", "ll", "ﾛﾝﾄﾞﾝｱﾍﾞﾆｭｰ", "ﾛﾝﾄﾞﾝ"],
        ),
        (
            "テレサレイク",
            Console::GBA,
            vec!["bbl", "bl", "ﾃﾚｻﾚｲｸ", "ﾚｲｸ", "ﾃﾚｲｸ"],
        ),
        (
            "ロックロックマウンテン",
            Console::_3DS,
            vec!["brrm", "rrm", "ﾛｯｸﾛｯｸﾏｳﾝﾃﾝ", "ﾛｸﾏ", "ﾛｯｸ", "岩山", "ﾛｯｸﾛｯｸ"],
        ),
        (
            "メイプルツリーハウス",
            Console::Wii,
            vec!["bmt", "mt", "ﾒｲﾌﾟﾙﾂﾘｰﾊｳｽ", "ﾒｲﾌﾟﾙ"],
        ),
        // ムーンカップ
        (
            "ベルリンシュトラーセ",
            Console::Tour,
            vec!["bbb", "ﾍﾞﾙﾘﾝｼｭﾄﾗｰｾ", "ﾍﾞﾙﾘﾝ"],
        ),
        (
            "ピーチガーデン",
            Console::DS,
            vec!["bpg", "pg", "ﾋﾟｰﾁｶﾞｰﾃﾞﾝ", "ﾋﾟﾁｶﾞ", "ｶﾞｰﾃﾞﾝ"],
        ),
        (
            "メリーメリーマウンテン",
            Console::New,
            vec![
                "bmm",
                "mm",
                "ﾒﾘｰﾒﾘｰﾏｳﾝﾃﾝ",
                "ﾒﾘﾏ",
                "ﾒﾘｰﾒﾘｰ",
                "ﾒﾘｰ",
                "ﾒﾘﾔﾏ",
                "ﾒﾘ山",
            ],
        ),
        (
            "レインボーロード",
            Console::_3DS,
            vec!["brr7", "rr7", "3dsﾆｼﾞ", "3ds虹", "7ﾆｼﾞ", "7虹"],
        ),
        // フルーツカップ
        (
            "アムステルダムブルーム",
            Console::Tour,
            vec![
                "bad",
                "ad",
                "amsterdam",
                "ｱﾑｽﾃﾙﾀﾞﾑﾌﾞﾙｰﾑ",
                "ｱﾑｽﾃﾙﾀﾞﾑ",
                "ｱﾑｽ",
                "ﾌﾞﾙｰﾑ",
            ],
        ),
        (
            "リバーサイドパーク",
            Console::GBA,
            vec!["brp", "rp", "ﾘﾊﾞｰｻｲﾄﾞﾊﾟｰｸ", "ﾘﾊﾞｰｻｲﾄﾞ", "ﾘﾊﾞﾊﾟ"],
        ),
        (
            "DKスノーボードクロス",
            Console::Wii,
            vec![
                "bdks",
                "dks",
                "summit",
                "dkｽﾉｰﾎﾞｰﾄﾞｸﾛｽ",
                "ｽﾉｰﾎﾞｰﾄﾞｸﾛｽ",
                "ｽﾉﾎﾞｸﾛｽ",
                "ｽﾉﾎﾞ",
            ],
        ),
        (
            "ヨッシーアイランド",
            Console::New,
            vec!["byi", "yi", "ﾖｯｼｰｱｲﾗﾝﾄﾞ", "ﾖｼｱｲ"],
        ),
        // ブーメランカップ
        (
            "バンコクラッシュ",
            Console::Tour,
            vec!["bbr", "br", "bangkok", "ﾊﾞﾝｺｸﾗｯｼｭ", "ﾊﾞﾝｺｸ"],
        ),
        (
            "マリオサーキット",
            Console::DS,
            vec!["bmc", "dsﾏﾘｵｻｰｷｯﾄ", "dsﾏﾘｻ"],
        ),
        (
            "ワルイージスタジアム",
            Console::GC,
            vec!["bws", "ﾜﾙｲｰｼﾞｽﾀｼﾞｱﾑ", "ﾜﾙｽﾀ"],
        ),
        (
            "シンガポールスプラッシュ",
            Console::Tour,
            vec![
                "bssy",
                "ssy",
                "bsis",
                "sis",
                "singapore",
                "ｼﾝｶﾞﾎﾟｰﾙｽﾌﾟﾗｯｼｭ",
                "ｼﾝｶﾞﾎﾟｰﾙ",
            ],
        ),
        // ハネカップ
        (
            "アテネポリス",
            Console::Tour,
            vec!["bada", "ada", "athens", "ｱﾃﾈﾎﾟﾘｽ", "ｱﾃﾈ"],
        ),
        (
            "デイジークルーザー",
            Console::GC,
            vec!["bdc", "dc", "ﾃﾞｲｼﾞｰｸﾙｰｻﾞｰ", "ﾃﾞｲｸﾙ"],
        ),
        (
            "ムーンリッジ&ハイウェイ",
            Console::Wii,
            vec!["bmh", "mh", "ﾑｰﾝﾘｯｼﾞ", "ﾑﾝﾊｲ", "ﾑｰﾝﾊｲ"],
        ),
        (
            "シャボンロード",
            Console::New,
            vec!["bscs", "scs", "ｼｬﾎﾞﾝﾛｰﾄﾞ", "ｼｬﾎﾞﾝ", "ｼｬﾎﾞﾛ"],
        ),
        // チェリーカップ
        (
            "ロサンゼルスコースト",
            Console::Tour,
            vec![
                "blal",
                "lal",
                "losangeles",
                "los",
                "ﾛｻﾝｾﾞﾙｽｺｰｽﾄ",
                "ﾛｻﾝｾﾞﾙｽ",
                "ﾛｽ",
            ],
        ),
        (
            "サンセットこうや",
            Console::GBA,
            vec!["bsw", "sw", "ｻﾝｾｯﾄｺｳﾔ", "ｻﾝｾｯﾄ", "ｺｳﾔ", "ｻﾝｾ"],
        ),
        (
            "ノコノコみさき",
            Console::Wii,
            vec!["bkc", "kc", "ﾉｺﾉｺﾐｻｷ", "ﾉｺﾉｺ", "ﾉｺﾐｻ", "ﾉｺﾐ"],
        ),
        (
            "バンクーバーバレー",
            Console::Tour,
            vec!["bvv", "vv", "vancouver", "ﾊﾞﾝｸｰﾊﾞｰﾊﾞﾚｰ", "ﾊﾞﾝｸｰﾊﾞｰ"],
        ),
        // ドングリカップ
	(
            "ローマアバンティ",
            Console::Tour,
            vec!["bra", "ra", "rome", "ﾛｰﾏｱﾊﾞﾝﾃｨ", "ﾛｰﾏ"],
        ),
	(
            "DKマウンテン",
            Console::GC,
            vec!["bdkm", "dkm", "dkﾏｳﾝﾃﾝ", "dkﾔﾏ", "dk山"],
        ),
	(
            "デイジーサーキット",
            Console::Wii,
            vec!["bdci", "dci", "ﾃﾞｲｼﾞｰｻｰｷｯﾄ", "ﾃﾞｲｻ"],
        ),
	(
            "パックンしんでん",
            Console::New,
            vec!["bppc", "ppc", "ﾊﾟｯｸﾝｼﾝﾃﾞﾝ", "ﾊﾟｸｼﾝ", "ｼﾝﾃﾞﾝ"],
        ),
        // トゲゾーカップ
	 (
            "マドリードグランデ",
            Console::Tour,
            vec!["bmd", "md", "madrid", "ﾏﾄﾞﾘｰﾄﾞｸﾞﾗﾝﾃﾞ", "ﾏﾄﾞﾘｰﾄﾞ"],
        ),
	 (
            "ロゼッタプラネット",
            Console::_3DS,
            vec!["briw", "riw", "ﾛｾﾞｯﾀﾌﾟﾗﾈｯﾄ", "ﾛｾﾞﾌﾟﾗ"],
        ),
	 (
            "クッパじょう3",
            Console::SFC,
            vec!["bbc3", "bbc3", "bc3", "ｸｯﾊﾟｼﾞｮｳ3", "ｸｯﾊﾟｼﾞｮｳ", "ｸｯﾊﾟ城"],
        ),
	 (
            "レインボーロード",
            Console::Wii,
            vec!["brr", "brrw", "rrw", "wiiﾆｼﾞ", "wiiﾚｲﾝﾎﾞｰﾛｰﾄﾞ", "wii虹", "ｳｨｰﾆｼﾞ"],
        ),
    ] {
        let course = Course::new(name.to_string(), console);
        let course_name = course.to_string();
        courses.push(course);
        for shorthand in shorthands {
            COURSE_SHORTHAND_MAP
                .lock()
                .unwrap()
                .insert(shorthand.to_string(), course_name.clone());
        }
    }
    Mutex::new(courses)
});

pub static COURSES_CONSOLE_MAP_MAP: Lazy<Mutex<HashMap<Console, HashMap<String, Course>>>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        for console in [
            Console::SFC,
            Console::GBA,
            Console::N64,
            Console::GC,
            Console::DS,
            Console::Wii,
            Console::_3DS,
            Console::New,
            Console::Tour,
        ] {
            let mut courses = HashMap::new();
            for course in COURSES.lock().unwrap().iter() {
                if course.console == console {
                    let noramlized_name = normalize_japanese_characters(course.name.clone());
                    courses.insert(noramlized_name, course.clone());
                }
            }
            map.insert(console, courses);
        }
        Mutex::new(map)
    });

pub static STRING_COURSE_MAP: Lazy<Mutex<HashMap<String, Course>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for course in COURSES.lock().unwrap().iter() {
        let noramlized_name = course.to_string();
        map.insert(noramlized_name, course.clone());
    }
    Mutex::new(map)
});

fn get_console_by_words(words: &Vec<Word>) -> Console {
    for word in words {
        let lower_text = word.text.to_lowercase();
        let lower_text = lower_text.trim();
        if lower_text.contains("sfc") {
            return Console::SFC;
        }
        if lower_text.contains("gba") {
            return Console::GBA;
        }
        if lower_text.contains("n64") {
            return Console::N64;
        }
        if lower_text.contains("gc") {
            return Console::GC;
        }
        // dsよりも3dsを先に判定する
        if lower_text.contains("3ds") {
            return Console::_3DS;
        }
        if lower_text.contains("ds") {
            return Console::DS;
        }
        if lower_text.contains("wii") {
            return Console::Wii;
        }
        if lower_text.contains("tour") {
            return Console::Tour;
        }
    }
    Console::New
}

pub fn get_course_by_words(words: &Vec<Word>) -> Option<Course> {
    let console = get_console_by_words(words);

    let binding = COURSES_CONSOLE_MAP_MAP.lock().unwrap();
    let course_map = binding.get(&console).unwrap();
    for word in words {
        let lower_text = word.text.to_lowercase();
        let lower_text = lower_text.trim();
        let lower_text = normalize_japanese_characters(lower_text.to_string());
        if let Some(course) = course_map.get(&lower_text) {
            return Some(course.clone());
        }
    }
    None
}

pub fn get_course_by_words_with_nearest(words: &Vec<Word>, threshold: usize) -> Option<Course> {
    if words.is_empty() {
        return None;
    }
    let console = get_console_by_words(words);

    let longest_word = words
        .iter()
        .max_by(|w1, w2| w1.text.len().cmp(&w2.text.len()))
        .unwrap();
    let normalized_longest_word = normalize_japanese_characters(longest_word.text.clone());

    let binding = COURSES_CONSOLE_MAP_MAP.lock().unwrap();
    let course_map = binding.get(&console).unwrap();
    let course_names = course_map.keys().collect::<Vec<&String>>();
    // レーベンシュタイン距離が最小のコースを探す
    let mut min_distance = std::usize::MAX;
    let mut min_course_name = None;
    for course_name in course_names {
        let normalized_course_name = normalize_japanese_characters(course_name.to_string());
        let distance = levenshtein(&normalized_longest_word, &normalized_course_name);
        if distance < min_distance {
            min_distance = distance;
            min_course_name = Some(course_name);
        }
    }
    if min_distance > threshold {
        return None;
    }
    if let Some(min_course_name) = min_course_name {
        return course_map.get(min_course_name).cloned();
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_japanese_characters() {
        assert_eq!(
            normalize_japanese_characters("あがぱ工EｅＥ".to_string()),
            "アカハエeee".to_string()
        );
    }

    fn vec_str_to_words(words: Vec<&str>) -> Vec<Word> {
        let mut vec = Vec::new();
        for word in words {
            vec.push(Word::new(word.to_string(), 0.0, 0.0, 0.0, 0.0));
        }
        vec
    }
    fn assert_vec_str_to_course(words: Vec<&str>, expected: Course) {
        let words = vec_str_to_words(words);
        let course = get_course_by_words(&words);
        assert_eq!(course, Some(expected));
    }
    fn assert_vec_str_to_course_with_nearest(words: Vec<&str>, expected: Option<Course>) {
        let words = vec_str_to_words(words);
        let course = get_course_by_words_with_nearest(&words, 3);
        assert_eq!(course, expected);
    }

    #[test]
    fn test_get_course_by_words() {
        assert_vec_str_to_course(
            vec!["ヨッシーサーキット", "GC"],
            Course::new("ヨッシーサーキット".to_string(), Console::GC),
        );
        assert_vec_str_to_course(
            vec!["ロックロックマウンテン", "3DS"],
            Course::new("ロックロックマウンテン".to_string(), Console::_3DS),
        );
        assert_vec_str_to_course(
            vec!["キノヒオサーキット", "3DS"],
            Course::new("キノピオサーキット".to_string(), Console::_3DS),
        );
        assert_vec_str_to_course(
            vec!["どうぶつの森"],
            Course::new("どうぶつの森".to_string(), Console::New),
        );
        assert_vec_str_to_course(
            vec!["どうぶつの森"],
            Course::new("どうぶつの森".to_string(), Console::New),
        );
    }

    #[test]
    fn test_get_course_by_words_with_nearest() {
        assert_vec_str_to_course_with_nearest(
            vec!["ヨッシーサーキット", "GC"],
            Some(Course::new("ヨッシーサーキット".to_string(), Console::GC)),
        );
        assert_vec_str_to_course_with_nearest(
            vec!["ックロックマウンテン", "3DS"],
            Some(Course::new(
                "ロックロックマウンテン".to_string(),
                Console::_3DS,
            )),
        );
        assert_vec_str_to_course_with_nearest(
            vec!["ノヒオサーキット", "3DS"],
            Some(Course::new("キノピオサーキット".to_string(), Console::_3DS)),
        );
        assert_vec_str_to_course_with_nearest(
            vec!["うぶつの森"],
            Some(Course::new("どうぶつの森".to_string(), Console::New)),
        );
        assert_vec_str_to_course_with_nearest(
            vec!["ーユーヨークドリーム", "Tour"],
            Some(Course::new(
                "ニューヨークドリーム".to_string(),
                Console::Tour,
            )),
        );
        assert_vec_str_to_course_with_nearest(vec!["あまりにもかけ離れている場合", "Tour"], None);
        assert_vec_str_to_course_with_nearest(vec!["キノコキャニオン"], None);
    }
}
