// https://projecteuler.net/problem=13

use num_bigint::BigUint;

pub fn make() -> crate::Problem {
    crate::Problem {
        title: "Large Sum",
        number: 13,
        solve: core_solve,
    }
}

fn core_solve() -> i64 {
    let nums = vec![
        BigUint::parse_bytes(b"37107287533902102798797998220837590246510135740250", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"46376937677490009712648124896970078050417018260538", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"74324986199524741059474233309513058123726617309629", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"91942213363574161572522430563301811072406154908250", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"23067588207539346171171980310421047513778063246676", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"89261670696623633820136378418383684178734361726757", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"28112879812849979408065481931592621691275889832738", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"44274228917432520321923589422876796487670272189318", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"47451445736001306439091167216856844588711603153276", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"70386486105843025439939619828917593665686757934951", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"62176457141856560629502157223196586755079324193331", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"64906352462741904929101432445813822663347944758178", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"92575867718337217661963751590579239728245598838407", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"58203565325359399008402633568948830189458628227828", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"80181199384826282014278194139940567587151170094390", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"35398664372827112653829987240784473053190104293586", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"86515506006295864861532075273371959191420517255829", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"71693888707715466499115593487603532921714970056938", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"54370070576826684624621495650076471787294438377604", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"53282654108756828443191190634694037855217779295145", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"36123272525000296071075082563815656710885258350721", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"45876576172410976447339110607218265236877223636045", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"17423706905851860660448207621209813287860733969412", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"81142660418086830619328460811191061556940512689692", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"51934325451728388641918047049293215058642563049483", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"62467221648435076201727918039944693004732956340691", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"15732444386908125794514089057706229429197107928209", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"55037687525678773091862540744969844508330393682126", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"18336384825330154686196124348767681297534375946515", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"80386287592878490201521685554828717201219257766954", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"78182833757993103614740356856449095527097864797581", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"16726320100436897842553539920931837441497806860984", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"48403098129077791799088218795327364475675590848030", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"87086987551392711854517078544161852424320693150332", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"59959406895756536782107074926966537676326235447210", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"69793950679652694742597709739166693763042633987085", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"41052684708299085211399427365734116182760315001271", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"65378607361501080857009149939512557028198746004375", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"35829035317434717326932123578154982629742552737307", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"94953759765105305946966067683156574377167401875275", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"88902802571733229619176668713819931811048770190271", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"25267680276078003013678680992525463401061632866526", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"36270218540497705585629946580636237993140746255962", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"24074486908231174977792365466257246923322810917141", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"91430288197103288597806669760892938638285025333403", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"34413065578016127815921815005561868836468420090470", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"23053081172816430487623791969842487255036638784583", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"11487696932154902810424020138335124462181441773470", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"63783299490636259666498587618221225225512486764533", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"67720186971698544312419572409913959008952310058822", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"95548255300263520781532296796249481641953868218774", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"76085327132285723110424803456124867697064507995236", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"37774242535411291684276865538926205024910326572967", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"23701913275725675285653248258265463092207058596522", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"29798860272258331913126375147341994889534765745501", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"18495701454879288984856827726077713721403798879715", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"38298203783031473527721580348144513491373226651381", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"34829543829199918180278916522431027392251122869539", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"40957953066405232632538044100059654939159879593635", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"29746152185502371307642255121183693803580388584903", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"41698116222072977186158236678424689157993532961922", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"62467957194401269043877107275048102390895523597457", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"23189706772547915061505504953922979530901129967519", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"86188088225875314529584099251203829009407770775672", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"11306739708304724483816533873502340845647058077308", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"82959174767140363198008187129011875491310547126581", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"97623331044818386269515456334926366572897563400500", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"42846280183517070527831839425882145521227251250327", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"55121603546981200581762165212827652751691296897789", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"32238195734329339946437501907836945765883352399886", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"75506164965184775180738168837861091527357929701337", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"62177842752192623401942399639168044983993173312731", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"32924185707147349566916674687634660915035914677504", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"99518671430235219628894890102423325116913619626622", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"73267460800591547471830798392868535206946944540724", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"76841822524674417161514036427982273348055556214818", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"97142617910342598647204516893989422179826088076852", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"87783646182799346313767754307809363333018982642090", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"10848802521674670883215120185883543223812876952786", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"71329612474782464538636993009049310363619763878039", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"62184073572399794223406235393808339651327408011116", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"66627891981488087797941876876144230030984490851411", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"60661826293682836764744779239180335110989069790714", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"85786944089552990653640447425576083659976645795096", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"66024396409905389607120198219976047599490197230297", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"64913982680032973156037120041377903785566085089252", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"16730939319872750275468906903707539413042652315011", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"94809377245048795150954100921645863754710598436791", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"78639167021187492431995700641917969777599028300699", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"15368713711936614952811305876380278410754449733078", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"40789923115535562561142322423255033685442488917353", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"44889911501440648020369068063960672322193204149535", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"41503128880339536053299340368006977710650566631954", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"81234880673210146739058568557934581403627822703280", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"82616570773948327592232845941706525094512325230608", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"22918802058777319719839450180888072429661980811197", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"77158542502016545090413245809786882778948721859617", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"72107838435069186155435662884062257473692284509516", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"20849603980134001723930671666823555245252804609722", 10)
            .expect("hard-coded"),
        BigUint::parse_bytes(b"53503534226472524250874054075591789781264330331690", 10)
            .expect("hard-coded"),
    ];

    nums.into_iter()
        .fold(BigUint::ZERO, |total, big_num| big_num + total) // sum all numbers
        .to_str_radix(10)[0..10] // only grab first 10 digits
        .as_bytes() // it's a base-ten number, so we know it's ASCII
        .iter()
        // convert string digits to u64
        .fold(0i64, |all_digits, digit| {
            all_digits * 10 + (digit - b'0') as i64
        })
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_answer() {
        assert_eq!((super::make().solve)(), 5537376230)
    }
}
