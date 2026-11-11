// SPDX-License-Identifier: MIT OR Apache-2.0
//
// tools/t4-tdc.mjs — the T4 generator (spec 006 Item 5; 001 §6.1, §6.4 P6).
//
// Usage:  node tools/t4-tdc.mjs <library directory> > <fixture>.csv
//
// Loads <library directory>/dist/index.js (the tibetan-date-calculator
// library, docs/sources.md) and writes to stdout one row for every Gregorian
// day from 1900-01-01 to 2100-12-31 inclusive:
//
//   gregorian_date,tibetan_year,month,is_leap_month,tshes
//
// Diagnostics (flag checks, shape counts) go to stderr; nothing goes to stdout
// unless every check passed.
//
// The zone (F14 (a), ruled 2026-11-11; spec 006 ADDENDUM 1, R4). The library
// reads the machine's time zone: dist/index.js:344 (`unixFromJulian`) adds
// `new Date().getTimezoneOffset()` -- the offset of the zone NOW -- to every
// Gregorian date it returns, and the date string is then formed by the local-time
// getters of `getDateStr` (lines 72-77), which use the zone's offset AT THAT
// DATE. In a zone whose offset has changed by about 12 hours inside 1900-2100
// the two disagree and the library gives other dates (measured by the halt of
// spec 006: under TZ=Pacific/Kiritimati every date before 1995 comes out one
// day early). So this generator sets its own zone to UTC before it loads the
// library and before it creates any Date, then checks that the zone holds, and
// refuses to write anything if it does not:
//   check 1: Intl.DateTimeFormat().resolvedOptions().timeZone is "UTC";
//   check 2: new Date().getTimezoneOffset() is 0;
//   check 3: getTimezoneOffset() is 0 for a Date at 12:00 UTC on every Gregorian
//            day the generator enumerates (and on every day of 1898-2102).
// A failed check writes nothing to stdout, names itself on stderr and exits 4.
// The clock's zone is read only in order to refuse; nothing from it reaches
// stdout. Under the pin the offset the library reads is 0 at every instant, so
// the clock's value never reaches the output. The runs of this generator under
// three caller zones therefore show that the generator does not depend on the
// caller's zone; they show nothing about the library, whose zone dependence the
// pin hides by construction.
//
// Why Tibetan components and not Gregorian ones. The library's Gregorian ->
// Tibetan route takes only a date STRING (dist/index.js lines 740-743; the
// README, line 32, warns that strings give zone-dependent results) or the
// clock. Its Tibetan -> Gregorian route takes explicit numbers: year, month,
// leap flag, day. This generator walks every Tibetan month of the relevant
// years through the library's own year -> months -> days enumeration
// (TibetanYear.getMonths, TibetanMonth.getDays) and takes the Gregorian date the
// library returns for each day. It passes no date string and never asks for
// "today". It does not modify the library.
//
// The library's year numbering (dist/index.js:12, YEAR_DIFF = 127): its
// "Tibetan year number" is the Western year + 127, so Janson's Y (eq. (4.1),
// p. 8, the Gregorian year in which the Tibetan year begins) = library year -
// 127. The fixture's `tibetan_year` is Janson's Y.
//
// The library's own flags, as read from dist/index.js lines 404-409, per
// NUMBER (not per calendar day):
//   isDoubledDay      the number is repeated: both of its days carry it
//   isLeapDay         the FIRST of the two days (the library's "leap day";
//                     Janson v1 p. 20 calls the first of the two the leap day)
//   isSkippedDay      the number occurs on no calendar day
//   isPreviousSkipped the number before this one (t - 1; 30 of the previous
//                     month for t = 1) is skipped
// These are checked here against what consecutive rows imply (METHOD rule 35:
// an oracle that disagrees with itself is not an oracle).

import { createRequire } from 'node:module';
import path from 'node:path';

const YEAR_DIFF = 127; // dist/index.js:12
const FIRST_Y = 1898; // enumerate a margin round the range, then cut
const LAST_Y = 2101;
const RANGE_FIRST = '1900-01-01';
const RANGE_LAST = '2100-12-31';
const ZONE_SPAN_FIRST = '1898-01-01'; // check 3's span, a superset of what is enumerated
const ZONE_SPAN_LAST = '2102-12-31';

const err = (s) => process.stderr.write(`${s}\n`);

// The pin: before the library is loaded and before any Date is created.
process.env.TZ = 'UTC';

// A failed zone check: nothing on stdout, the check named on stderr, exit 4.
const refuse = (check, text) => {
    err(`REFUSED: zone ${check} failed: ${text}`);
    err('nothing was written to stdout');
    process.exit(4);
};

const resolvedZone = Intl.DateTimeFormat().resolvedOptions().timeZone;
if (resolvedZone !== 'UTC') {
    refuse('check 1', `Intl.DateTimeFormat().resolvedOptions().timeZone is ${JSON.stringify(resolvedZone)}, not "UTC"`);
}
const offsetNow = new Date().getTimezoneOffset();
if (offsetNow !== 0) {
    refuse('check 2', `new Date().getTimezoneOffset() is ${offsetNow}, not 0`);
}

// Day numbers by UTC arithmetic on explicit components: no zone, no clock.
const dayNumber = (iso) => {
    const [y, m, d] = iso.split('-').map(Number);
    return Date.UTC(y, m - 1, d) / 86400000;
};
const isoOfDayNumber = (n) => new Date(n * 86400000).toISOString().slice(0, 10);

// Check 3: getTimezoneOffset() is 0 for a Date at 12:00 UTC on each given day.
const zoneCheck3 = (isoDates, label) => {
    let bad = 0;
    let first = null;
    for (const iso of isoDates) {
        const [y, m, d] = iso.split('-').map(Number);
        const offset = new Date(Date.UTC(y, m - 1, d, 12)).getTimezoneOffset();
        if (offset !== 0) {
            bad++;
            first ??= `${iso}: offset ${offset}`;
        }
    }
    if (bad) refuse('check 3', `${bad} days of ${label} have a non-zero offset at 12:00 UTC; the first is ${first}`);
};
const spanDays = [];
for (let n = dayNumber(ZONE_SPAN_FIRST); n <= dayNumber(ZONE_SPAN_LAST); n++) spanDays.push(isoOfDayNumber(n));
zoneCheck3(spanDays, `${ZONE_SPAN_FIRST}..${ZONE_SPAN_LAST}`);

const libDir = process.argv[2];
if (!libDir) {
    err('usage: node t4-tdc.mjs <library directory>');
    process.exit(2);
}
const lib = createRequire(import.meta.url)(path.resolve(libDir, 'dist', 'index.js'));
const monthKey = (r) => `${r.year}/${r.month}/${r.leap}`;

const problems = [];
const problem = (text) => {
    if (problems.length < 25) problems.push(text);
    problems.count = (problems.count ?? 0) + 1;
};

// 1. Enumerate every day of every month, through the library.
const rows = [];
const skippedNumbers = []; // [monthKey, tshes] the library calls skipped
let absentNotFlagged = 0;
for (let y = FIRST_Y; y <= LAST_Y; y++) {
    for (const month of new lib.TibetanYear(y + YEAR_DIFF).getMonths()) {
        const days = month.getDays();
        const present = new Set();
        for (const d of days) {
            present.add(d.date);
            rows.push({
                date: d.westernDateStr,
                year: d.year - YEAR_DIFF,
                month: d.month,
                leap: d.monthObj.isLeapMonth,
                tshes: d.date,
                isDoubledDay: d.isDoubledDay,
                isLeapDay: d.isLeapDay,
                isPreviousSkipped: d.isPreviousSkipped,
                isSkippedDay: d.isSkippedDay,
            });
        }
        for (let t = 1; t <= 30; t++) {
            if (present.has(t)) continue;
            const absent = new lib.TibetanDate({
                year: month.year,
                month: month.month,
                isLeapMonth: month.isLeapMonth,
                day: t,
                isLeapDay: false,
            });
            if (absent.isSkippedDay) {
                skippedNumbers.push([`${month.year - YEAR_DIFF}/${month.month}/${month.isLeapMonth}`, t]);
            } else {
                absentNotFlagged++;
                problem(`number ${t} of ${month.year - YEAR_DIFF}/${month.month}/${month.isLeapMonth} is absent from getDays() but not flagged isSkippedDay`);
            }
        }
    }
}

// 2. Chronological order, contiguity. And check 3 again, on the days the
// generator actually enumerated (a superset check: every one must lie in the
// span checked above), still before any output.
const enumeratedDates = rows.map((r) => r.date);
if (enumeratedDates.some((iso) => iso < ZONE_SPAN_FIRST || iso > ZONE_SPAN_LAST)) {
    refuse('check 3', `an enumerated date lies outside ${ZONE_SPAN_FIRST}..${ZONE_SPAN_LAST}`);
}
zoneCheck3(enumeratedDates, `the ${enumeratedDates.length} enumerated days`);
rows.sort((a, b) => dayNumber(a.date) - dayNumber(b.date));
for (let i = 1; i < rows.length; i++) {
    const gap = dayNumber(rows[i].date) - dayNumber(rows[i - 1].date);
    if (gap !== 1) problem(`not contiguous: ${rows[i - 1].date} then ${rows[i].date} (gap ${gap})`);
}

// 3. The library's flags against what consecutive rows imply.
let flagDoubledRows = 0;
let flagLeapDayRows = 0;
let flagPrevSkippedRows = 0;
let flagPrevSkippedOnSecond = 0; // a skipped number directly before a repeated one: both of its days carry the flag
let impliedDoubledRows = 0;
let impliedLeapDayRows = 0;
let impliedSkipTransitions = 0;
let disagreeing = 0;
for (let i = 1; i < rows.length - 1; i++) {
    const r = rows[i];
    const prev = rows[i - 1];
    const next = rows[i + 1];
    const sameAsNext = monthKey(next) === monthKey(r) && next.tshes === r.tshes;
    const sameAsPrev = monthKey(prev) === monthKey(r) && prev.tshes === r.tshes;
    const impliedDoubled = sameAsNext || sameAsPrev;
    const impliedLeapDay = sameAsNext;
    // the nearest earlier row with a different number
    let j = i - 1;
    while (j > 0 && monthKey(rows[j]) === monthKey(r) && rows[j].tshes === r.tshes) j--;
    const p = rows[j];
    const missing = monthKey(p) === monthKey(r) ? r.tshes - p.tshes - 1 : 30 - p.tshes + (r.tshes - 1);
    if (missing !== 0 && missing !== 1) problem(`${r.date}: ${missing} numbers missing between ${p.date} (${p.tshes}) and ${r.date} (${r.tshes})`);
    const impliedPrevSkipped = missing === 1;
    if (impliedDoubled) impliedDoubledRows++;
    if (impliedLeapDay) impliedLeapDayRows++;
    if (impliedPrevSkipped && !sameAsPrev) impliedSkipTransitions++;
    if (r.isDoubledDay) flagDoubledRows++;
    if (r.isLeapDay) flagLeapDayRows++;
    if (r.isPreviousSkipped) flagPrevSkippedRows++;
    if (r.isPreviousSkipped && sameAsPrev) flagPrevSkippedOnSecond++;
    if (r.isSkippedDay) problem(`${r.date}: a row with isSkippedDay true`);
    if (r.isDoubledDay !== impliedDoubled || r.isLeapDay !== impliedLeapDay || r.isPreviousSkipped !== impliedPrevSkipped) {
        disagreeing++;
        problem(
            `${r.date} ${monthKey(r)} tshes ${r.tshes}: library (doubled ${r.isDoubledDay}, leapDay ${r.isLeapDay}, previousSkipped ${r.isPreviousSkipped}) ` +
                `vs rows (doubled ${impliedDoubled}, leapDay ${impliedLeapDay}, previousSkipped ${impliedPrevSkipped})`,
        );
    }
}

// 4. Cut to the range, shape counts.
const range = rows.filter((r) => r.date >= RANGE_FIRST && r.date <= RANGE_LAST);
const expectedDays = dayNumber(RANGE_LAST) - dayNumber(RANGE_FIRST) + 1;
if (range.length !== expectedDays) problem(`range holds ${range.length} rows, expected ${expectedDays}`);
if (range[0].date !== RANGE_FIRST || range[range.length - 1].date !== RANGE_LAST) problem('range ends are wrong');

const cycleOf = (y) => Math.ceil((y - 1026) / 60); // Janson eq. (4.3), p. 8: only to count cycle boundaries
const monthsSeen = new Set();
const leapMonthsSeen = new Set();
let repeated = 0;
let skippedTransitions = 0;
let rollovers = 0;
let cycleBoundaries = 0;
for (let i = 0; i < range.length; i++) {
    const r = range[i];
    monthsSeen.add(monthKey(r));
    if (r.leap) leapMonthsSeen.add(monthKey(r));
    if (i === 0) continue;
    const p = range[i - 1];
    if (monthKey(p) === monthKey(r) && p.tshes === r.tshes) repeated++;
    else if (monthKey(p) === monthKey(r) ? r.tshes - p.tshes === 2 : 30 - p.tshes + (r.tshes - 1) === 1) skippedTransitions++;
    if (p.month === 12 && r.month === 1) rollovers++;
    if (cycleOf(p.year) !== cycleOf(r.year)) cycleBoundaries++;
}

err(`zone: pinned to ${resolvedZone} (check 1), offset now ${offsetNow} (check 2), offset 0 on ${spanDays.length} days of ${ZONE_SPAN_FIRST}..${ZONE_SPAN_LAST} and on all ${enumeratedDates.length} enumerated days (check 3)`);
err(`rows: ${range.length} (expected ${expectedDays}: ${RANGE_FIRST}..${RANGE_LAST})`);
err(`library flags over ${rows.length - 2} checked rows of the enumeration (${FIRST_Y}..${LAST_Y}): isDoubledDay true ${flagDoubledRows} (rows imply ${impliedDoubledRows}), isLeapDay true ${flagLeapDayRows} (rows imply ${impliedLeapDayRows}), isPreviousSkipped true ${flagPrevSkippedRows} (${flagPrevSkippedOnSecond} of them on the second day of a repeated number, ${flagPrevSkippedRows - flagPrevSkippedOnSecond} on a first or single day)`);
err(`library skipped numbers (isSkippedDay on a number absent from getDays): ${skippedNumbers.length}; skip transitions implied by the rows: ${impliedSkipTransitions}; absent but not flagged: ${absentNotFlagged}`);
err(`flag disagreements: ${disagreeing}`);
err(`shapes in range: months ${monthsSeen.size}, leap months ${leapMonthsSeen.size}, repeated tshes (second occurrences) ${repeated}, skipped tshes ${skippedTransitions}, 12->1 rollovers ${rollovers}, rabjung-cycle boundaries ${cycleBoundaries}`);

const show = (r) => `${r.date},${r.year},${r.month},${r.leap},${r.tshes}`;
err(`ends: first rows ${range.slice(0, 2).map(show).join(' | ')}; last rows ${range.slice(-2).map(show).join(' | ')}`);

if (problems.count) {
    err(`FAIL: ${problems.count} problems; the first ${problems.length}:`);
    for (const p of problems) err(`  ${p}`);
    process.exitCode = 3;
} else {
    let out = 'gregorian_date,tibetan_year,month,is_leap_month,tshes\n';
    for (const r of range) out += `${r.date},${r.year},${r.month},${r.leap},${r.tshes}\n`;
    process.stdout.write(out);
}
