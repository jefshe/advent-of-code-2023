#![feature(let_chains)]
use itertools::Itertools;
use std::cmp::min;
use std::collections::VecDeque;

use std::{collections::HashMap, ops::Range};

type Transformation = HashMap<Range<u64>, u64>;

fn main() {

   let (mut input, tfs) = parse_all();
   for tf in tfs {
    // sort input by ranges
    let sorted_input: VecDeque<Range<u64>> = input.into_iter()
        .sorted_by(|a,b| Ord::cmp(&a.start, &b.start))
        .collect();
    input = perform_tf_range(sorted_input, &tf);
   }
   println!("solution: {}", input.iter().map(|rng| rng.start).min().unwrap())
}

fn parse_all() -> (VecDeque<Range<u64>>, Vec<Transformation>) {
   let mut iter = BIG_INPUT.into_iter().peekable();
   let seeds = parse_seeds(iter.next().unwrap());
   iter.next(); // Skip empty line

   let mut tfs: Vec<Transformation> = Vec::new();
   while let Some(_) = iter.peek() {
    let map = parse_transformation(&mut iter);
    tfs.push(map);
   }

   let input: VecDeque<Range<u64>> = seeds
    .chunks(2)
    .map(|a| a[0]..a[0]+a[1])
    .collect();

    return (input, tfs)
}

fn parse_seeds(line: &str) -> Vec<u64> {
    line.split(" ").into_iter().skip(1).map(|seed| seed.parse().unwrap() ).collect()
}

fn parse_transformation<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Transformation {
    iter.next(); // skip title - we don't care
    let mut map: Transformation = HashMap::new();
    while let Some(line) = iter.next() && !line.trim().is_empty() {
        let row: Vec<u64> = line.split(" ").into_iter().map(|num| num.parse().unwrap()).collect();
        map.insert(row[1]..(row[1]+row[2]), row[0]);
    }
    map
}

fn perform_tf_range(mut a_vec: VecDeque<Range<u64>>, tf: &Transformation) -> VecDeque<Range<u64>> {
    let mut output: VecDeque<Range<u64>> = VecDeque::new();
    let mut b_vec: VecDeque<&Range<u64>> = tf.keys()
        .into_iter()
        .sorted_by(|a,b| Ord::cmp(&a.start, &b.start))
        .collect();
    loop {
        match(a_vec.pop_front(), b_vec.pop_front()) {
            (Some(a), Some(b)) => {
                if a.end <= b.start {
                    b_vec.push_front(b); // place b back on the queue
                    output.push_back(a.start..a.end);
                } else if a.start >= b.end {
                    // check the next b
                    a_vec.push_front(a);
                } else if a.start < b.start {
                    a_vec.push_front(b.start..a.end);
                    b_vec.push_front(b);
                    output.push_back(a.start..b.start);
                } else {
                    if a.end > b.end {
                        a_vec.push_front(b.end..a.end)
                    } else if b.end > a.end {
                        b_vec.push_front(b)
                    }
                    let base = tf.get(b).unwrap().to_owned();
                    let output_range = (base + (a.start - b.start))..(base + (min(a.end, b.end) - b.start));
                    output.push_back(output_range);
                }
            },
            (Some(a), None) => {
                output.push_back(a.start..a.end);
            },
            (None, _) => break
        }
    }
    output
}

const BIG_INPUT: [&str; 207] = [
"seeds: 104847962 3583832 1212568077 114894281 3890048781 333451605 1520059863 217361990 310308287 12785610 3492562455 292968049 1901414562 516150861 2474299950 152867148 3394639029 59690410 862612782 176128197",
"",
"seed-to-soil map:",
"2023441036 2044296880 396074363",
"2419515399 3839972576 454994720",
"274688417 699823315 258919718",
"533608135 0 431744151",
"965352286 431744151 161125324",
"3391658630 2936663910 903308666",
"200749950 1177785526 73938467",
"2874510119 1440389999 315892137",
"1916089471 2440371243 20593195",
"0 977035576 200749950",
"1936682666 1957538510 86758370",
"1440389999 2902130623 34533287",
"1126477610 592869475 106953840",
"3190402256 1756282136 201256374",
"1474923286 2460964438 441166185",
"1233431450 958743033 18292543",
"",
"soil-to-fertilizer map:",
"1479837493 1486696129 480988794",
"3637384566 3730606485 267472485",
"70483107 174821741 921411492",
"3586173142 3071290434 51211424",
"1960826287 1166716340 319979789",
"3952751562 3398939385 283772589",
"0 1096233233 70483107",
"1166716340 1967684923 313121153",
"3904857051 3682711974 47894511",
"2902018973 3122501858 276437527",
"991894599 0 174821741",
"3416901681 2902018973 169271461",
"3178456500 3998078970 238445181",
"",
"fertilizer-to-water map:",
"4274676882 2765984054 20290414",
"3642266392 2324011621 382224743",
"3159410287 4157769177 137198119",
"3437898965 2786274468 204367427",
"2136710407 1497332580 94233249",
"4121681656 2706236364 59747690",
"2362529584 1912615374 411396247",
"4181429346 3961317447 93247536",
"270199336 152334204 473273308",
"1977000445 3853228854 108088593",
"2085089038 1304420652 51621369",
"1407624846 2990641895 345516575",
"908901118 638701782 307729932",
"3296608406 1356042021 141290559",
"743472644 625607512 13094270",
"2230943656 3721642926 131585928",
"0 946431714 270199336",
"2773925831 3336158470 385484456",
"4024491135 1815424853 97190521",
"756566914 0 152334204",
"1304420652 4054564983 103204194",
"1753141421 1591565829 223859024",
"",
"water-to-light map:",
"139728365 0 27290780",
"4161521920 2345099742 65970280",
"3549264451 2411070022 15588060",
"846553766 4012820620 62155872",
"3276913175 3215861697 30588309",
"7256118 139495191 27523954",
"3653026602 3908255344 104565276",
"2007806695 3246450006 21131889",
"426542603 2677155019 292403006",
"3265259672 4167732422 11653503",
"2959802238 3267581895 77570040",
"2766290955 2562319792 114835227",
"779480399 3352881787 3460872",
"718945609 995514267 33385697",
"908709638 1675494216 164534584",
"1547970680 1643941067 31553149",
"1579523829 2009572806 335526936",
"782941271 2969558025 63612495",
"752331306 465171569 27149093",
"237284175 1454682639 189258428",
"3757591878 3724399189 59699158",
"2881126182 3784098347 78676056",
"1520219770 2426658082 27750910",
"3564852511 1366508548 88174091",
"2383999106 3681454004 42945185",
"4287237444 3345151935 7729852",
"2242562254 3649815914 31638090",
"3307501484 753751300 241762967",
"34780072 27290780 104948293",
"2028938584 3033170520 182691177",
"3037372278 237284175 227887394",
"3817291036 3356342659 5821939",
"0 132239073 7256118",
"1360895538 1266240382 51601463",
"3823112975 492320662 230498145",
"2274200344 1840028800 109798762",
"4053611120 2454408992 107910800",
"2426944291 1317841845 48666703",
"1073244222 3362164598 287651316",
"1457977942 1203998554 62241828",
"2591192365 1028899964 175098590",
"1412497001 3862774403 45480941",
"2475610994 4179385925 115581371",
"4227492200 1949827562 59745244",
"1915050765 4074976492 92755930",
"2211629761 722818807 30932493",
"",
"light-to-temperature map:",
"3741602262 2758947303 142653736",
"628739598 2901601039 50811783",
"1842260329 1084521599 145122645",
"2990409993 3493390513 390865485",
"4190333929 4159289690 83510514",
"984282519 2952412822 202948629",
"1826968660 3155361451 15291669",
"1329953386 2288178120 328046945",
"830513455 3304469747 74190775",
"1187231148 2616225065 142722238",
"2705943734 836690664 247830935",
"3381275478 1332397059 281233458",
"679551381 605080109 150962074",
"1987382974 756042183 44013157",
"4159289690 4263923057 31044239",
"1772730322 3250231409 54238338",
"2549609472 2131843858 156334262",
"1658000331 3378660522 114729991",
"298584178 195831363 330155420",
"3662508936 525986783 79093326",
"4273844443 4242800204 21122853",
"2953774669 800055340 36635324",
"2031396131 1613630517 518213341",
"265644560 162891745 32939618",
"162891745 1229644244 102752815",
"904704230 3170653120 79578289",
"",
"temperature-to-humidity map:",
"671484955 1144907174 532089323",
"1414132335 1960778188 125717021",
"2631474761 2586973888 1058655511",
"1991131055 744338927 221864400",
"2212995455 192320896 28611061",
"192320896 2186552402 224875394",
"2241895588 966203327 169532208",
"1744764149 431490014 67442815",
"1812206964 2106058626 80493776",
"516655377 1135735535 9171639",
"1559412773 1775426812 185351376",
"2622156499 2577655626 9318262",
"1203574278 220931957 210558057",
"3690130272 4094120212 156346211",
"3946017132 3645629399 348950164",
"525827016 598680988 145657939",
"1892700740 1676996497 98430315",
"3846476483 3994579563 99540649",
"2577655626 4250466423 44500873",
"1539849356 2086495209 19563417",
"2241606516 598391916 289072",
"417196290 498932829 99459087",
"",
"humidity-to-location map:",
"547577859 2546258172 54451455",
"2564186976 3913248498 28610653",
"2460249359 129990669 103937617",
"257798579 3257354132 21143365",
"511274864 3365252234 24536388",
"412475023 3389788622 98799841",
"2843712442 3615348771 251219053",
"0 2984989380 24111266",
"1074541266 4051128852 126947592",
"3109497668 265707418 9056683",
"3268495430 1450483293 18654108",
"1298820860 954974424 392849235",
"1691670095 1469137401 93027519",
"3886041222 2131196811 39611681",
"2797706800 1347823659 46005642",
"3767841776 233928286 19835810",
"743957513 253764096 11943322",
"4068806743 3941859151 109269701",
"2754107115 1910801576 16288912",
"1223331059 2244121482 75489801",
"3118554351 1393829301 56653992",
"3094931495 3009100646 14566173",
"3991994741 893524279 61450145",
"692486724 2192650693 51470789",
"1008564686 2952439802 32549578",
"535811252 3603582164 11766607",
"1784697614 2698363640 44335967",
"755900835 2353038285 193219887",
"278941944 1777268497 133533079",
"3925652903 3488588463 66341838",
"3558101581 2742699607 209740195",
"949120722 3305808270 59443964",
"2624116446 0 129990669",
"1939852817 274764101 520396542",
"3175208343 2037909724 93287087",
"3287149538 2600709627 97654013",
"650681177 1562164920 41805547",
"3787677586 795160643 98363636",
"1829033581 1927090488 110819236",
"4053444886 3897886641 15361857",
"1201488858 2170808492 21842201",
"3384803551 1603970467 173298030",
"2770396027 3278497497 27310773",
"1041114264 2319611283 33427002",
"602029314 3554930301 48651863",
"2592797629 3866567824 31318817",
"24111266 3023666819 233687313",
];