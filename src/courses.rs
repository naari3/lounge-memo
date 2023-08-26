use std::{collections::HashMap, fmt::Display};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use strsim::levenshtein;

use crate::word::{normalize_japanese_characters, Word};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Series {
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

impl Display for Series {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Series::SFC => write!(f, "SFC"),
            Series::GBA => write!(f, "GBA"),
            Series::N64 => write!(f, "N64"),
            Series::GC => write!(f, "GC"),
            Series::DS => write!(f, "DS"),
            Series::Wii => write!(f, "Wii"),
            Series::_3DS => write!(f, "3DS"),
            Series::New => write!(f, ""),
            Series::Tour => write!(f, "Tour"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Course {
    name: String,
    series: Series,
}

impl Course {
    pub fn new(name: String, series: Series) -> Self {
        Self { name, series }
    }
}

impl Display for Course {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // もしSeriesがNewだったら、シリーズ名を表示しない
        // それ以外の場合はスペースを間に挟んでシリーズ名を表示する
        if self.series == Series::New {
            write!(f, "{}", self.name)
        } else {
            write!(f, "{} {}", self.series, self.name)
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
    for (name, series, shorthands) in [
        // https://github.com/sheat-git/mk8dx.py/blob/main/mk8dx/data.py#L311
        // キノコカップ
        (
            "マリオカートスタジアム",
            Series::New,
            vec!["mks", "ﾏﾘｵｶｰﾄｽﾀｼﾞｱﾑ", "ﾏﾘｶｽ"],
        ),
        (
            "ウォーターパーク",
            Series::New,
            vec!["wp", "ｳｫｰﾀﾊﾟｰｸ", "ｦｰﾀｰﾊﾟｰｸ", "ｳｫﾀﾊﾟ", "ｦﾀﾊﾟ", "ｵﾀﾊﾟ"],
        ),
        (
            "スイーツキャニオン",
            Series::New,
            vec!["ssc", "ｽｲｰﾂｷｬﾆｵﾝ", "ｽｲｷｬﾆ"],
        ),
        (
            "ドッスンいせき",
            Series::New,
            vec!["tr", "ﾄﾞｯｽﾝｲｾｷ", "ﾄﾞｯｽﾝ", "ｲｾｷ", "ﾄﾞｯｽﾝ遺跡", "遺跡"],
        ),
        // フラワーカップ
        (
            "マリオサーキット",
            Series::New,
            vec!["mc", "ﾏﾘｵｻｰｷｯﾄ", "ﾏﾘｻ", "新ﾏﾘｻ", "ｼﾝﾏﾘｻ"],
        ),
        (
            "キノピオハーバー",
            Series::New,
            vec!["th", "ｷﾉﾋﾟｵﾊｰﾊﾞｰ", "ﾊｰﾊﾞｰ"],
        ),
        (
            "ねじれマンション",
            Series::New,
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
            Series::New,
            vec!["sgf", "ﾍｲﾎｰｺｳｻﾞﾝ", "ﾍｲﾎｰ鉱山", "ﾍｲｺｰ", "ﾍｲｺｳ", "ﾍｲ鉱"],
        ),
        // スターカップ
        (
            "サンシャインくうこう",
            Series::New,
            vec!["sa", "ｻﾝｼｬｲﾝｸｳｺｳ", "空港", "ｸｳｺｳ", "ｻﾝｼｬｲﾝ"],
        ),
        (
            "ドルフィンみさき",
            Series::New,
            vec!["ds", "ﾄﾞﾙﾌｨﾝﾐｻｷ", "ﾄﾞﾙﾐ", "ﾐｻｷ", "ﾄﾞﾙﾌｨﾝ岬", "岬"],
        ),
        (
            "エレクトロドリーム",
            Series::New,
            vec!["ed", "ｴﾚｸﾄﾛﾄﾞﾘｰﾑ", "ｴﾚﾄﾞ", "ｴﾚﾄﾞﾘ"],
        ),
        (
            "ワリオスノーマウンテン",
            Series::New,
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
            Series::New,
            vec!["cc", "ｽｶｲｶﾞｰﾃﾞﾝ", "ｽｶｶﾞ"],
        ),
        (
            "ホネホネさばく",
            Series::New,
            vec!["bdd", "ﾎﾈﾎﾈｻﾊﾞｸ", "ﾎﾈｻﾊﾞ", "ﾎﾈﾎﾈ"],
        ),
        (
            "クッパキャッスル",
            Series::New,
            vec!["bc", "ｸｯﾊﾟｷｬｯｽﾙ", "ｸﾊﾟｷｬ", "ｸｷｬﾊﾟ", "ｸｯｷｬﾊﾟｯｽﾙ"],
        ),
        (
            "レインボーロード",
            Series::New,
            vec!["rr", "ﾚｲﾝﾎﾞｰﾛｰﾄﾞ", "新虹", "ｼﾝﾆｼﾞ"],
        ),
        // たまごカップ
        (
            "ヨッシーサーキット",
            Series::GC,
            vec!["dyc", "yc", "ﾖｯｼｰｻｰｷｯﾄ", "ﾖｼｻ"],
        ),
        (
            "エキサイトバイク",
            Series::New,
            vec!["dea", "ea", "ｴｷｻｲﾄﾊﾞｲｸ", "ｴｷﾊﾞ"],
        ),
        (
            "ドラゴンロード",
            Series::New,
            vec!["ddd", "dd", "ﾄﾞﾗｺﾞﾝﾛｰﾄﾞ", "ﾄﾞﾗﾛ"],
        ),
        (
            "ミュートシティ",
            Series::New,
            vec!["dmc", "ﾐｭｰﾄｼﾃｨ", "ﾐｭｰﾄ"],
        ),
        // どうぶつカップ
        (
            "ベビィパーク",
            Series::GC,
            vec!["dbp", "bp", "ﾍﾞﾋﾞｨﾊﾟｰｸ", "ﾍﾞﾋﾞｰﾊﾟｰｸ", "ﾍﾞﾋﾞﾊﾟ"],
        ),
        (
            "チーズランド",
            Series::GBA,
            vec!["dcl", "cl", "ﾁｰｽﾞﾗﾝﾄﾞ", "ﾁｰｽﾞ"],
        ),
        (
            "ネイチャーロード",
            Series::New,
            vec!["dww", "ww", "ﾈｲﾁｬｰﾗﾝﾄﾞ", "ﾈｲﾁｬｰ", "ﾅﾁｭﾚ"],
        ),
        (
            "どうぶつの森",
            Series::New,
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
            Series::Wii,
            vec!["rmmm", "mmm", "ﾓｰﾓｰｶﾝﾄﾘｰ", "ﾓﾓｶﾝ", "ﾓｰﾓｰ"],
        ),
        (
            "マリオサーキット",
            Series::GBA,
            vec!["rmc", "gba", "ｸﾞﾊﾞ", "gbaﾏﾘｵｻｰｷｯﾄ", "gbaﾏﾘｻ"],
        ),
        (
            "プクプクビーチ",
            Series::DS,
            vec!["rccb", "ccb", "ﾌﾟｸﾌﾟｸﾋﾞｰﾁ", "ﾌﾟｸﾌﾟｸ", "ﾌﾟｸﾋﾞ"],
        ),
        (
            "キノピオハイウェイ",
            Series::N64,
            vec!["rtt", "tt", "ｷﾉﾋﾟｵﾊｲｳｪｲ", "ﾊｲｳｪｲ"],
        ),
        // バナナカップ
        (
            "カラカラさばく",
            Series::GC,
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
            Series::SFC,
            vec!["rdp3", "rdp", "dp3", "ﾄﾞｰﾅﾂﾍｲﾔ", "ﾍｲﾔ", "ﾄﾞｰﾅﾂ平野", "平野"],
        ),
        (
            "ピーチサーキット",
            Series::N64,
            vec!["rrry", "rry", "ﾋﾟｰﾁｻｰｷｯﾄ", "ﾋﾟﾁｻ"],
        ),
        (
            "DKジャングル",
            Series::_3DS,
            vec!["rdkj", "dk", "dkj", "dkｼﾞｬﾝｸﾞﾙ", "ｼﾞｬﾝｸﾞﾙ"],
        ),
        // このはカップ
        (
            "ワリオスタジアム",
            Series::DS,
            vec!["rws", "ws", "ﾜﾘｵｽﾀｼﾞｱﾑ", "ﾜﾘｽﾀ"],
        ),
        (
            "シャーベットランド",
            Series::GC,
            vec!["rsl", "sl", "ｼｬｰﾍﾞｯﾄﾗﾝﾄﾞ", "ｼｬｰﾍﾞｯﾄ", "ｼｬﾍﾞﾗﾝ", "ｼｬﾍﾞ"],
        ),
        (
            "ミュージックパーク",
            Series::_3DS,
            vec!["rmp", "mp", "ﾐｭｰｼﾞｯｸﾊﾟｰｸ", "ﾐｭｰﾊﾟ"],
        ),
        (
            "ヨッシーバレー",
            Series::N64,
            vec!["ryv", "yv", "ﾖｯｼｰﾊﾞﾚｰ", "ﾖｼﾊﾞ"],
        ),
        // サンダーカップ
        (
            "チクタクロック",
            Series::DS,
            vec!["rttc", "ttc", "ﾁｸﾀｸﾛｯｸ", "ﾁｸﾀｸ"],
        ),
        (
            "パックンスライダー",
            Series::_3DS,
            vec!["rpps", "pps", "ﾊﾟｯｸﾝｽﾗｲﾀﾞｰ", "ﾊﾟｸｽﾗ", "ﾊﾟｯｸﾝ"],
        ),
        (
            "グラグラかざん",
            Series::Wii,
            vec!["rgv", "gv", "ｸﾞﾗｸﾞﾗｶｻﾞﾝ", "ｸﾞﾗｸﾞﾗ", "ｶｻﾞﾝ"],
        ),
        (
            "レインボーロード",
            Series::N64,
            vec!["rrrd", "rrd", "64ﾚｲﾝﾎﾞｰﾛｰﾄﾞ", "64ﾆｼﾞ", "64虹", "ﾛｸﾖﾝ"],
        ),
        // ゼルダカップ
        (
            "ワリオこうざん",
            Series::Wii,
            vec!["dwgm", "wgm", "ﾜﾘｵｺｳｻﾞﾝ", "ﾜﾘｺｳ", "ﾜﾘｵ鉱山", "ﾜﾘ鉱"],
        ),
        (
            "レインボーロード",
            Series::SFC,
            vec!["drr", "sfcﾆｼﾞ", "sfcﾚｲﾝﾎﾞｰﾛｰﾄﾞ", "sfc虹", "sfc"],
        ),
        (
            "ツルツルツイスター",
            Series::New,
            vec!["diio", "iio", "ﾂﾙﾂﾙﾂｲｽﾀｰ", "ﾂﾂﾂ", "ﾂﾙﾂﾙ"],
        ),
        (
            "ハイラルサーキット",
            Series::New,
            vec!["dhc", "hc", "ﾊｲﾗﾙｻｰｷｯﾄ", "ﾊｲﾗﾙ"],
        ),
        // ベルカップ
        (
            "ネオクッパシティ",
            Series::_3DS,
            vec!["dnbc", "nbc", "ﾈｵｸｯﾊﾟｼﾃｨ", "ﾈｵﾊﾟ", "ﾈｵｸｯﾊﾟ"],
        ),
        (
            "リボンロード",
            Series::GBA,
            vec!["drir", "rir", "ﾘﾎﾞﾝﾛｰﾄﾞ", "ﾘﾎﾞﾝ"],
        ),
        (
            "リンリンメトロ",
            Series::New,
            vec!["dsbs", "sbs", "ﾘﾝﾘﾝﾒﾄﾛ", "ﾘﾝﾒﾄ"],
        ),
        ("ビッグブルー", Series::New, vec!["dbb", "bb", "ﾋﾞｯｸﾞﾌﾞﾙｰ"]),
        // パワフルカップ
        (
            "パリプロムナード",
            Series::Tour,
            vec!["bpp", "pp", "paris", "ﾊﾟﾘﾌﾟﾛﾑﾅｰﾄﾞ", "ﾊﾟﾘ"],
        ),
        (
            "キノピオサーキット",
            Series::_3DS,
            vec!["btc", "tc", "ｷﾉﾋﾟｵｻｰｷｯﾄ", "ｷﾉｻ"],
        ),
        (
            "チョコマウンテン",
            Series::N64,
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
            Series::Wii,
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
            Series::Tour,
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
            Series::DS,
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
            Series::GBA,
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
            Series::New,
            vec!["bnh", "nh", "ﾆﾝﾆﾝﾄﾞｰｼﾞｮｰ", "ﾆﾝｼﾞｮｰ", "ﾆﾝﾆﾝ"],
        ),
        // カブカップ
        (
            "ニューヨークドリーム",
            Series::Tour,
            vec!["bnym", "nym", "ﾆｭｰﾖｰｸﾄﾞﾘｰﾑ", "ﾆｭｰﾖｰｸ", "ﾆｭｰﾄﾞﾘ", "ny"],
        ),
        (
            "マリオサーキット3",
            Series::SFC,
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
            Series::N64,
            vec!["bkd", "kd", "64ｶﾗｻﾊﾞ", "64ｶﾗ", "64ｻﾊﾞ"],
        ),
        (
            "ワルイージピンボール",
            Series::DS,
            vec!["bwp", "ﾜﾙｲｰｼﾞﾋﾟﾝﾎﾞｰﾙ", "ﾜﾙﾋﾟﾝ", "ﾋﾟﾝﾎﾞｰﾙ"],
        ),
        // プロペラカップ
        (
            "シドニーサンシャイン",
            Series::Tour,
            vec!["bss", "ss", "bsys", "sys", "ｼﾄﾞﾆｰｻﾝｼｬｲﾝ", "ｼﾄﾞﾆｰ"],
        ),
        ("スノーランド", Series::GBA, vec!["bsl", "ｽﾉｰﾗﾝﾄﾞ", "ｽﾉﾗﾝ"]),
        (
            "キノコキャニオン",
            Series::Wii,
            vec!["bmg", "mg", "ｷﾉｺｷｬﾆｵﾝ", "ｷﾉｷｬﾆ", "ｷｬﾆｵﾝ"],
        ),
        (
            "アイスビルディング",
            Series::New,
            vec!["bshs", "shs", "ｱｲｽﾋﾞﾙﾃﾞｨﾝｸﾞ", "ｱｲｽ"],
        ),
        // ゴロいわカップ
        (
            "ロンドンアベニュー",
            Series::Tour,
            vec!["bll", "ll", "ﾛﾝﾄﾞﾝｱﾍﾞﾆｭｰ", "ﾛﾝﾄﾞﾝ"],
        ),
        (
            "テレサレイク",
            Series::GBA,
            vec!["bbl", "bl", "ﾃﾚｻﾚｲｸ", "ﾚｲｸ", "ﾃﾚｲｸ"],
        ),
        (
            "ロックロックマウンテン",
            Series::_3DS,
            vec!["brrm", "rrm", "ﾛｯｸﾛｯｸﾏｳﾝﾃﾝ", "ﾛｸﾏ", "ﾛｯｸ", "岩山", "ﾛｯｸﾛｯｸ"],
        ),
        (
            "メイプルツリーハウス",
            Series::Wii,
            vec!["bmt", "mt", "ﾒｲﾌﾟﾙﾂﾘｰﾊｳｽ", "ﾒｲﾌﾟﾙ"],
        ),
        // ムーンカップ
        (
            "ベルリンシュトラーセ",
            Series::Tour,
            vec!["bbb", "ﾍﾞﾙﾘﾝｼｭﾄﾗｰｾ", "ﾍﾞﾙﾘﾝ"],
        ),
        (
            "ピーチガーデン",
            Series::DS,
            vec!["bpg", "pg", "ﾋﾟｰﾁｶﾞｰﾃﾞﾝ", "ﾋﾟﾁｶﾞ", "ｶﾞｰﾃﾞﾝ"],
        ),
        (
            "メリーメリーマウンテン",
            Series::New,
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
            Series::_3DS,
            vec!["brr", "3dsﾆｼﾞ", "3ds虹", "7ﾆｼﾞ", "7虹"],
        ),
        // フルーツカップ
        (
            "アムステルダムブルーム",
            Series::Tour,
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
            Series::GBA,
            vec!["brp", "rp", "ﾘﾊﾞｰｻｲﾄﾞﾊﾟｰｸ", "ﾘﾊﾞｰｻｲﾄﾞ", "ﾘﾊﾞﾊﾟ"],
        ),
        (
            "DKスノーボードクロス",
            Series::Wii,
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
            Series::New,
            vec!["byi", "yi", "ﾖｯｼｰｱｲﾗﾝﾄﾞ", "ﾖｼｱｲ"],
        ),
        // ブーメランカップ
        (
            "バンコクラッシュ",
            Series::Tour,
            vec!["bbr", "br", "bangkok", "ﾊﾞﾝｺｸﾗｯｼｭ", "ﾊﾞﾝｺｸ"],
        ),
        (
            "マリオサーキット",
            Series::DS,
            vec!["bmc", "dsﾏﾘｵｻｰｷｯﾄ", "dsﾏﾘｻ"],
        ),
        (
            "ワルイージスタジアム",
            Series::GC,
            vec!["bws", "ﾜﾙｲｰｼﾞｽﾀｼﾞｱﾑ", "ﾜﾙｽﾀ"],
        ),
        (
            "シンガポールスプラッシュ",
            Series::Tour,
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
            Series::Tour,
            vec!["bada", "ada", "athens", "ｱﾃﾈﾎﾟﾘｽ", "ｱﾃﾈ"],
        ),
        (
            "デイジークルーザー",
            Series::GC,
            vec!["bdc", "dc", "ﾃﾞｲｼﾞｰｸﾙｰｻﾞｰ", "ﾃﾞｲｸﾙ"],
        ),
        (
            "ムーンリッジ&ハイウェイ",
            Series::Wii,
            vec!["bmh", "mh", "ﾑｰﾝﾘｯｼﾞ", "ﾑﾝﾊｲ", "ﾑｰﾝﾊｲ"],
        ),
        (
            "シャボンロード",
            Series::New,
            vec!["bscs", "scs", "ｼｬﾎﾞﾝﾛｰﾄﾞ", "ｼｬﾎﾞﾝ", "ｼｬﾎﾞﾛ"],
        ),
        // チェリーカップ
        (
            "ロサンゼルスコースト",
            Series::Tour,
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
            Series::GBA,
            vec!["bsw", "sw", "ｻﾝｾｯﾄｺｳﾔ", "ｻﾝｾｯﾄ", "ｺｳﾔ", "ｻﾝｾ"],
        ),
        (
            "ノコノコみさき",
            Series::Wii,
            vec!["bkc", "kc", "ﾉｺﾉｺﾐｻｷ", "ﾉｺﾉｺ", "ﾉｺﾐｻ", "ﾉｺﾐ"],
        ),
        (
            "バンクーバーバレー",
            Series::Tour,
            vec!["bvv", "vv", "vancouver", "ﾊﾞﾝｸｰﾊﾞｰﾊﾞﾚｰ", "ﾊﾞﾝｸｰﾊﾞｰ"],
        ),
        // ドングリカップ
        // TODO
        // トゲゾーカップ
        // TODO
    ] {
        let course = Course::new(name.to_string(), series);
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

pub static COURSES_SERIES_MAP_MAP: Lazy<Mutex<HashMap<Series, HashMap<String, Course>>>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        for series in [
            Series::SFC,
            Series::GBA,
            Series::N64,
            Series::GC,
            Series::DS,
            Series::Wii,
            Series::_3DS,
            Series::New,
            Series::Tour,
        ] {
            let mut courses = HashMap::new();
            for course in COURSES.lock().unwrap().iter() {
                if course.series == series {
                    let noramlized_name = normalize_japanese_characters(course.name.clone());
                    courses.insert(noramlized_name, course.clone());
                }
            }
            map.insert(series, courses);
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

fn get_series_by_words(words: &Vec<Word>) -> Series {
    for word in words {
        let lower_text = word.text.to_lowercase();
        let lower_text = lower_text.trim();
        if lower_text.contains("sfc") {
            return Series::SFC;
        }
        if lower_text.contains("gba") {
            return Series::GBA;
        }
        if lower_text.contains("n64") {
            return Series::N64;
        }
        if lower_text.contains("gc") {
            return Series::GC;
        }
        // dsよりも3dsを先に判定する
        if lower_text.contains("3ds") {
            return Series::_3DS;
        }
        if lower_text.contains("ds") {
            return Series::DS;
        }
        if lower_text.contains("wii") {
            return Series::Wii;
        }
        if lower_text.contains("tour") {
            return Series::Tour;
        }
    }
    Series::New
}

pub fn get_course_by_words(words: &Vec<Word>) -> Option<Course> {
    let series = get_series_by_words(words);

    let binding = COURSES_SERIES_MAP_MAP.lock().unwrap();
    let course_map = binding.get(&series).unwrap();
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
    let series = get_series_by_words(words);

    let longest_word = words
        .iter()
        .max_by(|w1, w2| w1.text.len().cmp(&w2.text.len()))
        .unwrap();
    let normalized_longest_word = normalize_japanese_characters(longest_word.text.clone());

    let binding = COURSES_SERIES_MAP_MAP.lock().unwrap();
    let course_map = binding.get(&series).unwrap();
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
            Course::new("ヨッシーサーキット".to_string(), Series::GC),
        );
        assert_vec_str_to_course(
            vec!["ロックロックマウンテン", "3DS"],
            Course::new("ロックロックマウンテン".to_string(), Series::_3DS),
        );
        assert_vec_str_to_course(
            vec!["キノヒオサーキット", "3DS"],
            Course::new("キノピオサーキット".to_string(), Series::_3DS),
        );
        assert_vec_str_to_course(
            vec!["どうぶつの森"],
            Course::new("どうぶつの森".to_string(), Series::New),
        );
        assert_vec_str_to_course(
            vec!["どうぶつの森"],
            Course::new("どうぶつの森".to_string(), Series::New),
        );
    }

    #[test]
    fn test_get_course_by_words_with_nearest() {
        assert_vec_str_to_course_with_nearest(
            vec!["ヨッシーサーキット", "GC"],
            Some(Course::new("ヨッシーサーキット".to_string(), Series::GC)),
        );
        assert_vec_str_to_course_with_nearest(
            vec!["ックロックマウンテン", "3DS"],
            Some(Course::new(
                "ロックロックマウンテン".to_string(),
                Series::_3DS,
            )),
        );
        assert_vec_str_to_course_with_nearest(
            vec!["ノヒオサーキット", "3DS"],
            Some(Course::new("キノピオサーキット".to_string(), Series::_3DS)),
        );
        assert_vec_str_to_course_with_nearest(
            vec!["うぶつの森"],
            Some(Course::new("どうぶつの森".to_string(), Series::New)),
        );
        assert_vec_str_to_course_with_nearest(
            vec!["ーユーヨークドリーム", "Tour"],
            Some(Course::new(
                "ニューヨークドリーム".to_string(),
                Series::Tour,
            )),
        );
        assert_vec_str_to_course_with_nearest(vec!["あまりにもかけ離れている場合", "Tour"], None);
        assert_vec_str_to_course_with_nearest(vec!["キノコキャニオン"], None);
    }
}
