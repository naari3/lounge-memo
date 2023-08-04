use std::{collections::HashMap, fmt::Display};

use once_cell::sync::Lazy;
use std::sync::Mutex;
use strsim::levenshtein;

use crate::word::{normalize_japanese_characters, Word};

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

pub fn get_course_by_words_with_nearest(words: &Vec<Word>, threshold: usize) -> Option<Course> {
    if words.len() == 0 {
        return None;
    }
    let series = get_series_by_words(&words);

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
