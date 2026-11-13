//! `golden/buryat/` under the gate (spec 014 Item 2; pays `DEBT.md` D-37).
//!
//! Spec 013 transcribed three T1 calendars into `golden/buryat/` and checked their
//! format, step and meta presence with scripts that were not kept; nothing in the
//! gate read the directory. These tests do, in the ordinary gate (no `#[ignore]`):
//! the file list, the meta files, the loader's formats, the step check (`FORKS.md`
//! F28; 001 §6.3) and the month rule (`FORKS.md` F36). The engine is not called.
//!
//! **What these tests do not check (METHOD rule 38):** that a number is the one
//! printed on the calendar (spec 013's two passes did that; the blind re-read of
//! spec 014 Item 5 re-reads a sample), which computation system a file follows
//! (`q1_differential.rs`), or that a `.meta.yaml` is true (only that it is there,
//! has its keys in order and is non-empty). `golden/reference/` metas are not read
//! by anything (`DEBT.md` D-39).
//!
//! Each check has its refusal shown in this file, in memory, on a deliberately
//! wrong input (METHOD rule 31); the same defects were also planted in a
//! `git archive` copy of the commit and the suite run there (spec 014's report).

mod common;

use common::buryat::{
    BuryatRow, FILES, META_KEYS, SourceFile, add_days, csv_names, label_month, load,
    lunation_starts, meta_check, meta_names, month_rule, parse, parse_date, printed_label, read,
    step_check,
};
use common::fixtures::{day_number, iso};

fn stems_with(suffix: &str) -> Vec<String> {
    let mut names: Vec<String> = FILES
        .iter()
        .map(|f| format!("{}{suffix}", f.stem))
        .collect();
    names.sort();
    names
}

fn aginsky_2025() -> (SourceFile, Vec<BuryatRow>) {
    let file = FILES[1];
    (file, load(file.stem))
}

/// The error of a refusal, or a panic if the input was accepted.
fn refusal<T: std::fmt::Debug>(result: Result<T, String>) -> String {
    match result {
        Err(e) => e,
        Ok(v) => panic!("expected a refusal, got {v:?}"),
    }
}

#[test]
fn buryat_the_csv_files_are_the_three_the_tests_name() {
    // A CSV added to golden/buryat/ without a line in `common::buryat::FILES`
    // (and so without the checks below) makes this fail.
    assert_eq!(
        csv_names(),
        stems_with(".csv"),
        "the CSV files of golden/buryat/"
    );
    assert_eq!(FILES.len(), 3, "the files the tests name");
}

#[test]
fn buryat_every_csv_has_its_meta_file_and_every_meta_file_its_csv() {
    assert_eq!(
        meta_names(),
        stems_with(".meta.yaml"),
        "the meta files of golden/buryat/"
    );
}

#[test]
fn buryat_every_meta_file_has_the_readme_keys_in_order_and_none_empty() {
    for file in FILES {
        let name = format!("{}.meta.yaml", file.stem);
        if let Err(e) = meta_check(&name, &read(&name)) {
            panic!("{e}");
        }
    }
    assert_eq!(
        META_KEYS.len(),
        17,
        "golden/buryat/README.md's section \"The meta keys\""
    );
}

#[test]
fn buryat_every_csv_loads_with_the_formats_of_its_readme() {
    for file in FILES {
        let rows = load(file.stem);
        assert_eq!(rows.len(), file.rows, "{}: rows", file.stem);
        assert_eq!(
            rows.first().map(|r| iso(r.date)).as_deref(),
            Some(file.first),
            "{}: first date",
            file.stem
        );
        assert_eq!(
            rows.last().map(|r| iso(r.date)).as_deref(),
            Some(file.last),
            "{}: last date",
            file.stem
        );
    }
}

#[test]
fn buryat_the_tshes_steps_are_plus_1_plus_2_or_a_doubled_day_and_the_flags_agree() {
    for file in FILES {
        let name = format!("{}.csv", file.stem);
        if let Err(e) = step_check(&name, &load(file.stem)) {
            panic!("{e}");
        }
    }
}

#[test]
fn buryat_the_months_follow_the_lunation_rule_of_the_readme() {
    for file in FILES {
        let name = format!("{}.csv", file.stem);
        if let Err(e) = month_rule(&name, &load(file.stem), file.labelled) {
            panic!("{e}");
        }
    }
    // The shape the rule read: lunations per file (README: 15, 14, 14).
    let counts: Vec<usize> = FILES
        .iter()
        .map(|f| lunation_starts(&load(f.stem)).len())
        .collect();
    assert_eq!(counts, [15, 14, 14], "lunations per file");
}

#[test]
fn buryat_the_add_days_helper_inverts_the_day_number() {
    for n in [
        0, 1, 58, 59, 60, 365, 366, 730, 1461, 10_000, 36_524, 73_414,
    ] {
        let start = (1900, 1, 1);
        let moved = add_days(start, n);
        assert_eq!(
            day_number(moved),
            day_number(start) + n,
            "add_days(1900-01-01, {n}) = {}",
            iso(moved)
        );
    }
    assert_eq!(add_days((2024, 2, 28), 1), (2024, 2, 29));
    assert_eq!(add_days((2025, 12, 31), 1), (2026, 1, 1));
    assert_eq!(add_days((2025, 3, 1), -1), (2025, 2, 28));
}

/// The lines joined, each followed by a newline.
fn lines_text<'a>(lines: impl IntoIterator<Item = &'a str>) -> String {
    let mut out = String::new();
    for line in lines {
        out.push_str(line);
        out.push('\n');
    }
    out
}

/// Replaces line `n` (1 = the header) of `text` with `with`.
fn with_line(text: &str, n: usize, with: &str) -> String {
    let mut out = String::new();
    for (i, line) in text.lines().enumerate() {
        out.push_str(if i + 1 == n { with } else { line });
        out.push('\n');
    }
    out
}

#[test]
fn buryat_the_loader_refuses_each_defect_it_names() {
    let name = "plant.csv";
    let good = read("aginsky-datsan-2025.csv");
    assert!(parse(name, &good).is_ok(), "the unplanted text loads");
    let cells = |n: usize| -> Vec<String> {
        good.lines()
            .nth(n - 1)
            .unwrap_or("")
            .split(',')
            .map(str::to_owned)
            .collect()
    };
    // A data row (line 3 of the file) with one cell changed.
    let cell_changed = |index: usize, value: &str| -> String {
        let mut c = cells(3);
        c[index] = value.to_owned();
        with_line(&good, 3, &c.join(","))
    };
    let plants: Vec<(&str, String, &str)> = vec![
        (
            "a header other than 001 §6.2's seven columns",
            good.replacen("gregorian_date,tshes", "date,tshes", 1),
            "HEADER:",
        ),
        (
            "a row with an eighth field",
            with_line(
                &good,
                3,
                &format!("{},extra", good.lines().nth(2).unwrap_or("")),
            ),
            "FIELD-COUNT:",
        ),
        (
            "a row with six fields",
            with_line(&good, 3, "2025-01-02,3,12,false,false,"),
            "FIELD-COUNT:",
        ),
        (
            "a quote character",
            good.replacen("printed label", "printed \"label", 1),
            "QUOTE:",
        ),
        ("a carriage return", good.replacen('\n', "\r\n", 1), "CR:"),
        (
            "no final newline",
            good.trim_end().to_owned(),
            "FINAL-NEWLINE:",
        ),
        (
            "a date that is not ISO",
            cell_changed(0, "02.01.2025"),
            "DATE:",
        ),
        (
            "a day the month does not have",
            cell_changed(0, "2025-02-30"),
            "DATE:",
        ),
        (
            "a date that is not one day after the previous row",
            cell_changed(0, "2025-01-03"),
            "DATE-STEP:",
        ),
        ("a tshes of 0", cell_changed(1, "0"), "TSHES-RANGE:"),
        ("a tshes of 31", cell_changed(1, "31"), "TSHES-RANGE:"),
        (
            "a tshes that is not a number",
            cell_changed(1, "x"),
            "TSHES-RANGE:",
        ),
        ("a month of 13", cell_changed(2, "13"), "MONTH-RANGE:"),
        ("a month of 0", cell_changed(2, "0"), "MONTH-RANGE:"),
        ("is_leap_month yes", cell_changed(3, "yes"), "BOOL:"),
        ("is_doubled empty", cell_changed(4, ""), "BOOL:"),
        ("is_doubled True", cell_changed(4, "True"), "BOOL:"),
        ("a skipped of 31", cell_changed(5, "31"), "SKIPPED-RANGE:"),
        (
            "no data row",
            good.lines()
                .next()
                .map(|h| format!("{h}\n"))
                .unwrap_or_default(),
            "ZERO-ROWS:",
        ),
    ];
    for (what, text, code) in plants {
        let e = refusal(parse(name, &text));
        assert!(e.starts_with(code), "{what}: expected {code}, got {e}");
    }
    // The date parser itself.
    assert_eq!(parse_date("2024-02-29"), Ok((2024, 2, 29)));
    assert!(parse_date("2023-02-29").is_err(), "2023 is not a leap year");
    assert!(parse_date("2025-1-02").is_err(), "a one-digit month");
    // The empty is_leap_month the Ivolginsky file uses is accepted.
    assert!(parse(name, &read("ivolginsky-datsan-2021.csv")).is_ok());
}

#[test]
fn buryat_the_step_check_refuses_a_plus_3_step_and_each_flag_defect() {
    let name = "plant.csv";
    let (_, rows) = aginsky_2025();
    assert!(step_check(name, &rows).is_ok(), "the unplanted rows pass");
    // The first row that follows a skipped number, tshes raised by one: a step of +3.
    let skipped_at = rows
        .iter()
        .position(|r| r.skipped.is_some() && r.tshes < 30)
        .expect("a skipped row");
    let mut plant = rows.clone();
    plant[skipped_at].tshes += 1;
    let e = refusal(step_check(name, &plant));
    assert!(
        e.starts_with("STEP:") && e.contains("step +3"),
        "a +3 step: {e}"
    );
    // A doubled day whose flag is false.
    let doubled_at = rows.iter().position(|r| r.doubled).expect("a doubled row");
    let mut plant = rows.clone();
    plant[doubled_at].doubled = false;
    assert!(refusal(step_check(name, &plant)).starts_with("DOUBLED-FLAG:"));
    // A plain day flagged doubled.
    let mut plant = rows.clone();
    plant[3].doubled = true;
    assert!(refusal(step_check(name, &plant)).starts_with("DOUBLED-FLAG:"));
    // A skipped day with the wrong number.
    let mut plant = rows.clone();
    plant[skipped_at].skipped = plant[skipped_at].skipped.map(|s| s % 30 + 1);
    assert!(refusal(step_check(name, &plant)).starts_with("SKIPPED-VALUE:"));
    // The same tshes on three days running.
    let mut plant = rows.clone();
    plant[1].tshes = plant[0].tshes;
    plant[2].tshes = plant[0].tshes;
    plant[1].doubled = true;
    plant[2].doubled = true;
    assert!(refusal(step_check(name, &plant)).starts_with("DOUBLED-TWICE:"));
    // The first row carries a flag its predecessor cannot give.
    let mut plant = rows.clone();
    plant[0].skipped = Some(1);
    assert!(refusal(step_check(name, &plant)).starts_with("FIRST-ROW:"));
}

#[test]
fn buryat_the_month_rule_refuses_a_label_on_the_wrong_lunation_and_each_form_defect() {
    let name = "plant.csv";
    let (file, rows) = aginsky_2025();
    assert!(
        month_rule(name, &rows, file.labelled).is_ok(),
        "the unplanted rows pass"
    );
    let starts = lunation_starts(&rows);
    let range = |k: usize| starts[k]..starts.get(k + 1).copied().unwrap_or(rows.len());
    // A label moved to the neighbouring lunation: lunation 4 takes lunation 3's month and label.
    let mut plant = rows.clone();
    let (month, notes) = (rows[starts[2]].month, rows[starts[2]].notes.clone());
    for i in range(3) {
        plant[i].month = month;
    }
    plant[starts[3]].notes = notes;
    let e = refusal(month_rule(name, &plant, true));
    assert!(
        e.starts_with("LABEL-ORDER:"),
        "a label moved to the next lunation: {e}"
    );
    // A month changed inside a lunation.
    let mut plant = rows.clone();
    assert_eq!(
        rows[starts[2] + 5].month,
        Some(1),
        "the lunation's own month, before the plant"
    );
    plant[starts[2] + 5].month = Some(5);
    assert!(refusal(month_rule(name, &plant, true)).starts_with("LUNATION-LABEL:"));
    // The label note and the month disagree (the label is not moved with it).
    let mut plant = rows.clone();
    for i in range(3) {
        plant[i].month = Some(rows[starts[3]].month.unwrap_or(1) % 12 + 1);
    }
    assert!(refusal(month_rule(name, &plant, true)).starts_with("LABEL-NOTE:"));
    // The extra month with a number.
    let extra = rows
        .iter()
        .position(|r| r.leap == Some(true))
        .expect("the extra month");
    let mut plant = rows.clone();
    for i in range(
        starts
            .iter()
            .position(|&s| s == extra)
            .expect("a lunation start"),
    ) {
        plant[i].month = Some(12);
    }
    assert!(refusal(month_rule(name, &plant, true)).starts_with("LEAP-MONTH-FORM:"));
    // A lunation whose first row prints no label.
    let mut plant = rows.clone();
    plant[starts[4]].notes.clear();
    assert!(refusal(month_rule(name, &plant, true)).starts_with("LABEL-NOTE:"));
    // A month filled in a source that prints no label.
    let ivolginsky = load(FILES[0].stem);
    assert!(month_rule(FILES[0].stem, &ivolginsky, false).is_ok());
    let mut plant = ivolginsky.clone();
    plant[10].month = Some(1);
    assert!(refusal(month_rule(name, &plant, false)).starts_with("UNLABELLED:"));
}

#[test]
fn buryat_the_label_words_give_the_months_of_the_readme_seasonal_order() {
    // golden/buryat/README.md: «Первый весенний» 1, «Второй весенний» 2, «Третий весенний» 3,
    // «Первый летний» 4 to «Третий летний» 6, «Первый осенний» 7 to «Третий осенний» 9,
    // «Первый зимний» 10, «Второй зимний» 11, «Третий зимний» 12.
    let words = ["весенний", "летний", "осенний", "зимний"];
    let ordinals = ["Первый", "Второй", "Третий"];
    let mut expected = 1;
    for season in words {
        for ordinal in ordinals {
            assert_eq!(
                label_month(&format!("{ordinal} {season} месяц – тигр")),
                Some(expected),
                "{ordinal} {season}"
            );
            expected += 1;
        }
    }
    assert_eq!(label_month("Дополнительный зимний месяц"), None);
    assert_eq!(
        printed_label("lunation begins; printed label «Третий зимний месяц – корова»; x"),
        Some("Третий зимний месяц – корова")
    );
    assert_eq!(printed_label("no label here"), None);
}

#[test]
fn buryat_the_meta_check_refuses_a_missing_key_a_wrong_order_and_an_empty_value() {
    let name = "plant.meta.yaml";
    let good = read("aginsky-datsan-2025.meta.yaml");
    assert!(meta_check(name, &good).is_ok());
    // A key deleted.
    let without = lines_text(good.lines().filter(|l| !l.starts_with("lama:")));
    assert!(refusal(meta_check(name, &without)).starts_with("META-KEYS:"));
    // Two keys swapped.
    let mut lines: Vec<&str> = good.lines().collect();
    lines.swap(2, 3);
    let swapped = lines_text(lines);
    assert!(refusal(meta_check(name, &swapped)).starts_with("META-KEYS:"));
    // A value emptied.
    let emptied = lines_text(good.lines().map(|l| {
        if l.starts_with("rights:") {
            "rights:"
        } else {
            l
        }
    }));
    assert!(refusal(meta_check(name, &emptied)).starts_with("META-EMPTY:"));
    // An empty file.
    assert!(refusal(meta_check(name, "")).starts_with("META-KEYS:"));
}
