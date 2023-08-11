use windows::{
    core::Interface,
    Graphics::Imaging::{BitmapBufferAccessMode, BitmapPixelFormat, SoftwareBitmap},
    Media::Ocr::OcrEngine,
    Win32::System::WinRT::IMemoryBufferByteAccess,
};

#[derive(Debug)]
pub struct Word {
    pub text: String,
    pub x: f64,
    pub y: f64,
    pub height: f64,
    pub width: f64,
}

impl Word {
    pub fn new(text: String, x: f64, y: f64, height: f64, width: f64) -> Word {
        Word {
            text,
            x,
            y,
            height,
            width,
        }
    }
}

fn make_bmp(buffer: &[u8], width: i32, height: i32) -> anyhow::Result<SoftwareBitmap> {
    let bmp = SoftwareBitmap::Create(BitmapPixelFormat::Rgba8, width, height)?;
    {
        let bmp_buf = bmp.LockBuffer(BitmapBufferAccessMode::ReadWrite)?;
        let array: IMemoryBufferByteAccess = bmp_buf.CreateReference()?.cast()?;

        let mut data = std::ptr::null_mut();
        let mut capacity = 0;
        unsafe {
            array.GetBuffer(&mut data, &mut capacity)?;
        }
        assert_eq!((width * height * 4).abs(), capacity as i32);

        let slice = unsafe { std::slice::from_raw_parts_mut(data, capacity as usize) };
        slice.chunks_mut(4).enumerate().for_each(|(i, c)| {
            c[0] = buffer[3 * i];
            c[1] = buffer[3 * i + 1];
            c[2] = buffer[3 * i + 2];
            c[3] = 255;
        });
    }

    Ok(bmp)
}

pub async fn words_from_image_buffer(
    buffer: &[u8],
    width: i32,
    height: i32,
) -> anyhow::Result<Vec<Word>> {
    let bmp = make_bmp(buffer, width, height)?;
    let engine = OcrEngine::TryCreateFromUserProfileLanguages()?;
    let result = engine.RecognizeAsync(&bmp)?.await?.Lines()?;
    let mut collected_words: Vec<Word> = Vec::new();

    result.into_iter().for_each(|line| {
        let words = line.Words().unwrap();
        let line_text = line.Text().unwrap().to_string_lossy();
        let mut _x = 0.0f64;
        let mut _y = 0.0f64;
        let mut line_heigth = 0.0;
        let mut line_width = 0.0;
        let mut idx = 0;
        words.into_iter().for_each(|word| {
            let rect = word.BoundingRect().unwrap();
            let name = &word.Text().unwrap().to_string_lossy();
            collected_words.push(Word::new(
                name.to_string(),
                rect.X.into(),
                rect.Y.into(),
                rect.Height.into(),
                rect.Width.into(),
            ));
            if idx == 0 {
                _x = rect.X as f64;
            }
            if line_heigth < rect.Height as f64 {
                line_heigth = rect.Height as f64;
            }
            line_width += rect.Width as f64;
            if _y < rect.Y as f64 {
                _y = rect.Y as f64;
            }
            idx += 1;
        });
        collected_words.push(Word {
            x: _x,
            y: _y,
            text: line_text.replace(' ', ""),
            height: line_heigth,
            width: line_width,
        })
    });
    Ok(collected_words)
}

pub fn normalize_japanese_characters(text: String) -> String {
    let mut normalized = text;
    // 全角英数字を半角英数字に変換
    normalized = normalized.replace('０', "0");
    normalized = normalized.replace('１', "1");
    normalized = normalized.replace('２', "2");
    normalized = normalized.replace('３', "3");
    normalized = normalized.replace('４', "4");
    normalized = normalized.replace('５', "5");
    normalized = normalized.replace('６', "6");
    normalized = normalized.replace('７', "7");
    normalized = normalized.replace('８', "8");
    normalized = normalized.replace('９', "9");
    normalized = normalized.replace('Ａ', "A");
    normalized = normalized.replace('Ｂ', "B");
    normalized = normalized.replace('Ｃ', "C");
    normalized = normalized.replace('Ｄ', "D");
    normalized = normalized.replace('Ｅ', "E");
    normalized = normalized.replace('Ｆ', "F");
    normalized = normalized.replace('Ｇ', "G");
    normalized = normalized.replace('Ｈ', "H");
    normalized = normalized.replace('Ｉ', "I");
    normalized = normalized.replace('Ｊ', "J");
    normalized = normalized.replace('Ｋ', "K");
    normalized = normalized.replace('Ｌ', "L");
    normalized = normalized.replace('Ｍ', "M");
    normalized = normalized.replace('Ｎ', "N");
    normalized = normalized.replace('Ｏ', "O");
    normalized = normalized.replace('Ｐ', "P");
    normalized = normalized.replace('Ｑ', "Q");
    normalized = normalized.replace('Ｒ', "R");
    normalized = normalized.replace('Ｓ', "S");
    normalized = normalized.replace('Ｔ', "T");
    normalized = normalized.replace('Ｕ', "U");
    normalized = normalized.replace('Ｖ', "V");
    normalized = normalized.replace('Ｗ', "W");
    normalized = normalized.replace('Ｘ', "X");
    normalized = normalized.replace('Ｙ', "Y");
    normalized = normalized.replace('Ｚ', "Z");
    normalized = normalized.replace('ａ', "a");
    normalized = normalized.replace('ｂ', "b");
    normalized = normalized.replace('ｃ', "c");
    normalized = normalized.replace('ｄ', "d");
    normalized = normalized.replace('ｅ', "e");
    normalized = normalized.replace('ｆ', "f");
    normalized = normalized.replace('ｇ', "g");
    normalized = normalized.replace('ｈ', "h");
    normalized = normalized.replace('ｉ', "i");
    normalized = normalized.replace('ｊ', "j");
    normalized = normalized.replace('ｋ', "k");
    normalized = normalized.replace('ｌ', "l");
    normalized = normalized.replace('ｍ', "m");
    normalized = normalized.replace('ｎ', "n");
    normalized = normalized.replace('ｏ', "o");
    normalized = normalized.replace('ｐ', "p");
    normalized = normalized.replace('ｑ', "q");
    normalized = normalized.replace('ｒ', "r");
    normalized = normalized.replace('ｓ', "s");
    normalized = normalized.replace('ｔ', "t");
    normalized = normalized.replace('ｕ', "u");
    normalized = normalized.replace('ｖ', "v");
    normalized = normalized.replace('ｗ', "w");
    normalized = normalized.replace('ｘ', "x");
    normalized = normalized.replace('ｙ', "y");
    normalized = normalized.replace('ｚ', "z");
    normalized = normalized.replace('　', " ");
    // 濁点・半濁点つきの文字を濁点なしに変換
    normalized = normalized.replace('が', "か");
    normalized = normalized.replace('ぎ', "き");
    normalized = normalized.replace('ぐ', "く");
    normalized = normalized.replace('げ', "け");
    normalized = normalized.replace('ご', "こ");
    normalized = normalized.replace('ざ', "さ");
    normalized = normalized.replace('じ', "し");
    normalized = normalized.replace('ず', "す");
    normalized = normalized.replace('ぜ', "せ");
    normalized = normalized.replace('ぞ', "そ");
    normalized = normalized.replace('だ', "た");
    normalized = normalized.replace('ぢ', "ち");
    normalized = normalized.replace('づ', "つ");
    normalized = normalized.replace('で', "て");
    normalized = normalized.replace('ど', "と");
    normalized = normalized.replace('ば', "は");
    normalized = normalized.replace('び', "ひ");
    normalized = normalized.replace('ぶ', "ふ");
    normalized = normalized.replace('べ', "へ");
    normalized = normalized.replace('ぼ', "ほ");
    normalized = normalized.replace('ぱ', "は");
    normalized = normalized.replace('ぴ', "ひ");
    normalized = normalized.replace('ぷ', "ふ");
    normalized = normalized.replace('ぺ', "へ");
    normalized = normalized.replace('ぽ', "ほ");
    normalized = normalized.replace('ガ', "カ");
    normalized = normalized.replace('ギ', "キ");
    normalized = normalized.replace('グ', "ク");
    normalized = normalized.replace('ゲ', "ケ");
    normalized = normalized.replace('ゴ', "コ");
    normalized = normalized.replace('ザ', "サ");
    normalized = normalized.replace('ジ', "シ");
    normalized = normalized.replace('ズ', "ス");
    normalized = normalized.replace('ゼ', "セ");
    normalized = normalized.replace('ゾ', "ソ");
    normalized = normalized.replace('ダ', "タ");
    normalized = normalized.replace('ヂ', "チ");
    normalized = normalized.replace('ヅ', "ツ");
    normalized = normalized.replace('デ', "テ");
    normalized = normalized.replace('ド', "ト");
    normalized = normalized.replace('バ', "ハ");
    normalized = normalized.replace('ビ', "ヒ");
    normalized = normalized.replace('ブ', "フ");
    normalized = normalized.replace('ベ', "ヘ");
    normalized = normalized.replace('ボ', "ホ");
    normalized = normalized.replace('パ', "ハ");
    normalized = normalized.replace('ピ', "ヒ");
    normalized = normalized.replace('プ', "フ");
    normalized = normalized.replace('ペ', "ヘ");
    normalized = normalized.replace('ポ', "ホ");
    normalized = normalized.replace('ヴ', "ウ");
    normalized = normalized.replace('゛', "");
    normalized = normalized.replace('゜', "");
    // ひらがなをカタカナに変換
    normalized = normalized.replace('ぁ', "ァ");
    normalized = normalized.replace('ぃ', "ィ");
    normalized = normalized.replace('ぅ', "ゥ");
    normalized = normalized.replace('ぇ', "ェ");
    normalized = normalized.replace('ぉ', "ォ");
    normalized = normalized.replace('っ', "ッ");
    normalized = normalized.replace('ゃ', "ャ");
    normalized = normalized.replace('ゅ', "ュ");
    normalized = normalized.replace('ょ', "ョ");
    normalized = normalized.replace('ゎ', "ヮ");
    normalized = normalized.replace('あ', "ア");
    normalized = normalized.replace('い', "イ");
    normalized = normalized.replace('う', "ウ");
    normalized = normalized.replace('え', "エ");
    normalized = normalized.replace('お', "オ");
    normalized = normalized.replace('か', "カ");
    normalized = normalized.replace('き', "キ");
    normalized = normalized.replace('く', "ク");
    normalized = normalized.replace('け', "ケ");
    normalized = normalized.replace('こ', "コ");
    normalized = normalized.replace('さ', "サ");
    normalized = normalized.replace('し', "シ");
    normalized = normalized.replace('す', "ス");
    normalized = normalized.replace('せ', "セ");
    normalized = normalized.replace('そ', "ソ");
    normalized = normalized.replace('た', "タ");
    normalized = normalized.replace('ち', "チ");
    normalized = normalized.replace('つ', "ツ");
    normalized = normalized.replace('て', "テ");
    normalized = normalized.replace('と', "ト");
    normalized = normalized.replace('な', "ナ");
    normalized = normalized.replace('に', "ニ");
    normalized = normalized.replace('ぬ', "ヌ");
    normalized = normalized.replace('ね', "ネ");
    normalized = normalized.replace('の', "ノ");
    normalized = normalized.replace('は', "ハ");
    normalized = normalized.replace('ひ', "ヒ");
    normalized = normalized.replace('ふ', "フ");
    normalized = normalized.replace('へ', "ヘ");
    normalized = normalized.replace('ほ', "ホ");
    normalized = normalized.replace('ま', "マ");
    normalized = normalized.replace('み', "ミ");
    normalized = normalized.replace('む', "ム");
    normalized = normalized.replace('め', "メ");
    normalized = normalized.replace('も', "モ");
    normalized = normalized.replace('や', "ヤ");
    normalized = normalized.replace('ゆ', "ユ");
    normalized = normalized.replace('よ', "ヨ");
    normalized = normalized.replace('ら', "ラ");
    normalized = normalized.replace('り', "リ");
    normalized = normalized.replace('る', "ル");
    normalized = normalized.replace('れ', "レ");
    normalized = normalized.replace('ろ', "ロ");
    normalized = normalized.replace('わ', "ワ");
    normalized = normalized.replace('を', "ヲ");
    normalized = normalized.replace('ん', "ン");
    // 小文字カタカナを大文字カタカナに変換
    normalized = normalized.replace('ァ', "ア");
    normalized = normalized.replace('ィ', "イ");
    normalized = normalized.replace('ゥ', "ウ");
    normalized = normalized.replace('ェ', "エ");
    normalized = normalized.replace('ォ', "オ");
    normalized = normalized.replace('ッ', "ツ");
    normalized = normalized.replace('ャ', "ヤ");
    normalized = normalized.replace('ュ', "ユ");
    normalized = normalized.replace('ョ', "ヨ");
    normalized = normalized.replace('ヮ', "ワ");
    normalized = normalized.replace('ヵ', "カ");
    // 工→エ
    normalized = normalized.replace('工', "エ");

    normalized = normalized.to_lowercase();

    normalized
}
