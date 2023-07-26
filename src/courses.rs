use std::{collections::HashMap, fmt::Display};

use once_cell::sync::Lazy;
use std::sync::Mutex;

use crate::word::Word;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

pub static COURSES: Lazy<Mutex<Vec<Course>>> = Lazy::new(|| {
    let mut courses = Vec::new();
    for (name, series) in [
        // キノコカップ
        ("マリオカートスタジアム", Series::New),
        ("ウォーターパーク", Series::New),
        ("スイーツキャニオン", Series::New),
        ("ドッスンいせき", Series::New),
        // フラワーカップ
        ("マリオサーキット", Series::New),
        ("キノピオハーバー", Series::New),
        ("ねじれマンション", Series::New),
        ("ヘイホーこうざん", Series::New),
        // スターカップ
        ("サンシャインくうこう", Series::New),
        ("ドルフィンみさき", Series::New),
        ("エレクトロドリーム", Series::New),
        ("ワリオスノーマウンテン", Series::New),
        // スペシャルカップ
        ("スカイガーデン", Series::New),
        ("ホネホネさばく", Series::New),
        ("クッパキャッスル", Series::New),
        ("レインボーロード", Series::New),
        // たまごカップ
        ("ヨッシーサーキット", Series::GC),
        ("エキサイトバイク", Series::New),
        ("ドラゴンロード", Series::New),
        ("ミュートシティ", Series::New),
        // どうぶつカップ
        ("ベビィパーク", Series::GC),
        ("チーズランド", Series::GBA),
        ("ネイチャーロード", Series::New),
        ("どうぶつの森", Series::New),
        // こうらカップ
        ("モーモーカントリー", Series::Wii),
        ("マリオサーキット", Series::GBA),
        ("プクプクビーチ", Series::DS),
        ("キノピオハイウェイ", Series::N64),
        // バナナカップ
        ("カラカラさばく", Series::GC),
        ("ドーナツへいや3", Series::SFC),
        ("ピーチサーキット", Series::N64),
        ("DKジャングル", Series::_3DS),
        // このはカップ
        ("ワリオスタジアム", Series::DS),
        ("シャーベットランド", Series::GC),
        ("ミュージックパーク", Series::_3DS),
        ("ヨッシーバレー", Series::N64),
        // サンダーカップ
        ("チクタクロック", Series::DS),
        ("パックンスライダー", Series::_3DS),
        ("グラグラかざん", Series::Wii),
        ("レインボーロード", Series::N64),
        // ゼルダカップ
        ("ワリオこうざん", Series::Wii),
        ("レインボーロード", Series::SFC),
        ("ツルツルツイスター", Series::New),
        ("ハイラルサーキット", Series::New),
        // ベルカップ
        ("ネオクッパシティ", Series::_3DS),
        ("リボンロード", Series::GBA),
        ("リンリンメトロ", Series::New),
        ("ビッグブルー", Series::New),
        // パワフルカップ
        ("パリプロムナード", Series::Tour),
        ("キノピオサーキット", Series::_3DS),
        ("チョコマウンテン", Series::N64),
        ("ココナッツモール", Series::Wii),
        // まねきネコカップ
        ("トーキョースクランブル", Series::Tour),
        ("キノコリッジウェイ", Series::DS),
        ("スカイガーデン", Series::GBA),
        ("ニンニンドージョー", Series::New),
        // カブカップ
        ("ニューヨークドリーム", Series::Tour),
        ("マリオサーキット3", Series::SFC),
        ("カラカラさばく", Series::N64),
        ("ワルイージピンボール", Series::DS),
        // プロペラカップ
        ("シドニーサンシャイン", Series::Tour),
        ("スノーランド", Series::GBA),
        ("キノコキャニオン", Series::Wii),
        ("アイスビルディング", Series::New),
        // ゴロいわカップ
        ("ロンドンアベニュー", Series::Tour),
        ("テレサレイク", Series::GBA),
        ("ロックロックマウンテン", Series::_3DS),
        ("メイプルツリーハウス", Series::Wii),
        // ムーンカップ
        ("ベルリンシュトラーセ", Series::Tour),
        ("ピーチガーデン", Series::DS),
        ("メリーメリーマウンテン", Series::New),
        ("レインボーロード", Series::_3DS),
        // フルーツカップ
        ("アムステルダムブルーム", Series::Tour),
        ("リバーサイドパーク", Series::GBA),
        ("DKスノーボードクロス", Series::Wii),
        ("ヨッシーアイランド", Series::New),
        // ブーメランカップ
        ("バンコクラッシュ", Series::Tour),
        ("マリオサーキット", Series::DS),
        ("ワルイージスタジアム", Series::GC),
        ("シンガポールスプラッシュ", Series::Tour),
        // ハネカップ
        ("アテネポリス", Series::Tour),
        ("デイジークルーザー", Series::GC),
        ("ムーンリッジ&ハイウェイ", Series::Wii),
        ("シャボンロード", Series::New),
        // チェリーカップ
        ("ロサンゼルスコースト", Series::Tour),
        ("サンセットこうや", Series::GBA),
        ("ノコノコみさき", Series::Wii),
        ("バンクーバーバレー", Series::Tour),
        // ドングリカップ
        // TODO
        // トゲゾーカップ
        // TODO
    ] {
        courses.push(Course::new(name.to_string(), series));
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
    return Series::New;
}

pub fn get_course_by_words(words: &Vec<Word>) -> Option<Course> {
    let series = get_series_by_words(&words);

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
    return None;
}

pub fn normalize_japanese_characters(text: String) -> String {
    let mut normalized = text.replace("ー", "ー");
    // 全角英数字を半角英数字に変換
    normalized = normalized.replace("０", "0");
    normalized = normalized.replace("１", "1");
    normalized = normalized.replace("２", "2");
    normalized = normalized.replace("３", "3");
    normalized = normalized.replace("４", "4");
    normalized = normalized.replace("５", "5");
    normalized = normalized.replace("６", "6");
    normalized = normalized.replace("７", "7");
    normalized = normalized.replace("８", "8");
    normalized = normalized.replace("９", "9");
    normalized = normalized.replace("Ａ", "A");
    normalized = normalized.replace("Ｂ", "B");
    normalized = normalized.replace("Ｃ", "C");
    normalized = normalized.replace("Ｄ", "D");
    normalized = normalized.replace("Ｅ", "E");
    normalized = normalized.replace("Ｆ", "F");
    normalized = normalized.replace("Ｇ", "G");
    normalized = normalized.replace("Ｈ", "H");
    normalized = normalized.replace("Ｉ", "I");
    normalized = normalized.replace("Ｊ", "J");
    normalized = normalized.replace("Ｋ", "K");
    normalized = normalized.replace("Ｌ", "L");
    normalized = normalized.replace("Ｍ", "M");
    normalized = normalized.replace("Ｎ", "N");
    normalized = normalized.replace("Ｏ", "O");
    normalized = normalized.replace("Ｐ", "P");
    normalized = normalized.replace("Ｑ", "Q");
    normalized = normalized.replace("Ｒ", "R");
    normalized = normalized.replace("Ｓ", "S");
    normalized = normalized.replace("Ｔ", "T");
    normalized = normalized.replace("Ｕ", "U");
    normalized = normalized.replace("Ｖ", "V");
    normalized = normalized.replace("Ｗ", "W");
    normalized = normalized.replace("Ｘ", "X");
    normalized = normalized.replace("Ｙ", "Y");
    normalized = normalized.replace("Ｚ", "Z");
    normalized = normalized.replace("ａ", "a");
    normalized = normalized.replace("ｂ", "b");
    normalized = normalized.replace("ｃ", "c");
    normalized = normalized.replace("ｄ", "d");
    normalized = normalized.replace("ｅ", "e");
    normalized = normalized.replace("ｆ", "f");
    normalized = normalized.replace("ｇ", "g");
    normalized = normalized.replace("ｈ", "h");
    normalized = normalized.replace("ｉ", "i");
    normalized = normalized.replace("ｊ", "j");
    normalized = normalized.replace("ｋ", "k");
    normalized = normalized.replace("ｌ", "l");
    normalized = normalized.replace("ｍ", "m");
    normalized = normalized.replace("ｎ", "n");
    normalized = normalized.replace("ｏ", "o");
    normalized = normalized.replace("ｐ", "p");
    normalized = normalized.replace("ｑ", "q");
    normalized = normalized.replace("ｒ", "r");
    normalized = normalized.replace("ｓ", "s");
    normalized = normalized.replace("ｔ", "t");
    normalized = normalized.replace("ｕ", "u");
    normalized = normalized.replace("ｖ", "v");
    normalized = normalized.replace("ｗ", "w");
    normalized = normalized.replace("ｘ", "x");
    normalized = normalized.replace("ｙ", "y");
    normalized = normalized.replace("ｚ", "z");
    normalized = normalized.replace("　", " ");
    // 濁点・半濁点つきの文字を濁点なしに変換
    normalized = normalized.replace("が", "か");
    normalized = normalized.replace("ぎ", "き");
    normalized = normalized.replace("ぐ", "く");
    normalized = normalized.replace("げ", "け");
    normalized = normalized.replace("ご", "こ");
    normalized = normalized.replace("ざ", "さ");
    normalized = normalized.replace("じ", "し");
    normalized = normalized.replace("ず", "す");
    normalized = normalized.replace("ぜ", "せ");
    normalized = normalized.replace("ぞ", "そ");
    normalized = normalized.replace("だ", "た");
    normalized = normalized.replace("ぢ", "ち");
    normalized = normalized.replace("づ", "つ");
    normalized = normalized.replace("で", "て");
    normalized = normalized.replace("ど", "と");
    normalized = normalized.replace("ば", "は");
    normalized = normalized.replace("び", "ひ");
    normalized = normalized.replace("ぶ", "ふ");
    normalized = normalized.replace("べ", "へ");
    normalized = normalized.replace("ぼ", "ほ");
    normalized = normalized.replace("ぱ", "は");
    normalized = normalized.replace("ぴ", "ひ");
    normalized = normalized.replace("ぷ", "ふ");
    normalized = normalized.replace("ぺ", "へ");
    normalized = normalized.replace("ぽ", "ほ");
    normalized = normalized.replace("ガ", "カ");
    normalized = normalized.replace("ギ", "キ");
    normalized = normalized.replace("グ", "ク");
    normalized = normalized.replace("ゲ", "ケ");
    normalized = normalized.replace("ゴ", "コ");
    normalized = normalized.replace("ザ", "サ");
    normalized = normalized.replace("ジ", "シ");
    normalized = normalized.replace("ズ", "ス");
    normalized = normalized.replace("ゼ", "セ");
    normalized = normalized.replace("ゾ", "ソ");
    normalized = normalized.replace("ダ", "タ");
    normalized = normalized.replace("ヂ", "チ");
    normalized = normalized.replace("ヅ", "ツ");
    normalized = normalized.replace("デ", "テ");
    normalized = normalized.replace("ド", "ト");
    normalized = normalized.replace("バ", "ハ");
    normalized = normalized.replace("ビ", "ヒ");
    normalized = normalized.replace("ブ", "フ");
    normalized = normalized.replace("ベ", "ヘ");
    normalized = normalized.replace("ボ", "ホ");
    normalized = normalized.replace("パ", "ハ");
    normalized = normalized.replace("ピ", "ヒ");
    normalized = normalized.replace("プ", "フ");
    normalized = normalized.replace("ペ", "ヘ");
    normalized = normalized.replace("ポ", "ホ");
    normalized = normalized.replace("ヴ", "ウ");
    normalized = normalized.replace("゛", "");
    normalized = normalized.replace("゜", "");
    // ひらがなをカタカナに変換
    normalized = normalized.replace("ぁ", "ァ");
    normalized = normalized.replace("ぃ", "ィ");
    normalized = normalized.replace("ぅ", "ゥ");
    normalized = normalized.replace("ぇ", "ェ");
    normalized = normalized.replace("ぉ", "ォ");
    normalized = normalized.replace("っ", "ッ");
    normalized = normalized.replace("ゃ", "ャ");
    normalized = normalized.replace("ゅ", "ュ");
    normalized = normalized.replace("ょ", "ョ");
    normalized = normalized.replace("ゎ", "ヮ");
    normalized = normalized.replace("あ", "ア");
    normalized = normalized.replace("い", "イ");
    normalized = normalized.replace("う", "ウ");
    normalized = normalized.replace("え", "エ");
    normalized = normalized.replace("お", "オ");
    normalized = normalized.replace("か", "カ");
    normalized = normalized.replace("き", "キ");
    normalized = normalized.replace("く", "ク");
    normalized = normalized.replace("け", "ケ");
    normalized = normalized.replace("こ", "コ");
    normalized = normalized.replace("さ", "サ");
    normalized = normalized.replace("し", "シ");
    normalized = normalized.replace("す", "ス");
    normalized = normalized.replace("せ", "セ");
    normalized = normalized.replace("そ", "ソ");
    normalized = normalized.replace("た", "タ");
    normalized = normalized.replace("ち", "チ");
    normalized = normalized.replace("つ", "ツ");
    normalized = normalized.replace("て", "テ");
    normalized = normalized.replace("と", "ト");
    normalized = normalized.replace("な", "ナ");
    normalized = normalized.replace("に", "ニ");
    normalized = normalized.replace("ぬ", "ヌ");
    normalized = normalized.replace("ね", "ネ");
    normalized = normalized.replace("の", "ノ");
    normalized = normalized.replace("は", "ハ");
    normalized = normalized.replace("ひ", "ヒ");
    normalized = normalized.replace("ふ", "フ");
    normalized = normalized.replace("へ", "ヘ");
    normalized = normalized.replace("ほ", "ホ");
    normalized = normalized.replace("ま", "マ");
    normalized = normalized.replace("み", "ミ");
    normalized = normalized.replace("む", "ム");
    normalized = normalized.replace("め", "メ");
    normalized = normalized.replace("も", "モ");
    normalized = normalized.replace("や", "ヤ");
    normalized = normalized.replace("ゆ", "ユ");
    normalized = normalized.replace("よ", "ヨ");
    normalized = normalized.replace("ら", "ラ");
    normalized = normalized.replace("り", "リ");
    normalized = normalized.replace("る", "ル");
    normalized = normalized.replace("れ", "レ");
    normalized = normalized.replace("ろ", "ロ");
    normalized = normalized.replace("わ", "ワ");
    normalized = normalized.replace("を", "ヲ");
    normalized = normalized.replace("ん", "ン");
    // 小文字カタカナを大文字カタカナに変換
    normalized = normalized.replace("ァ", "ア");
    normalized = normalized.replace("ィ", "イ");
    normalized = normalized.replace("ゥ", "ウ");
    normalized = normalized.replace("ェ", "エ");
    normalized = normalized.replace("ォ", "オ");
    normalized = normalized.replace("ッ", "ツ");
    normalized = normalized.replace("ャ", "ヤ");
    normalized = normalized.replace("ュ", "ユ");
    normalized = normalized.replace("ョ", "ヨ");
    normalized = normalized.replace("ヮ", "ワ");
    normalized = normalized.replace("ヵ", "カ");
    // 工→エ
    normalized = normalized.replace("工", "エ");

    normalized = normalized.to_lowercase();

    return normalized;
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

    #[test]
    fn test_get_course_by_words() {
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
    }
}
