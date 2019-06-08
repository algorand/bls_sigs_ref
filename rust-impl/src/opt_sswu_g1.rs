/*!
Hashing to G1.
*/

use pairing::bls12_381::Fq;
use ff::Field;

/* *** addchain for 1000602388805416848354447456433976039139220704984751971333014534031007912622709466110671907282253916009473568139946 *** */
/* Bos-Coster (win=4) : 458 links, 16 variables */
/// Addition chain implementing exponentiation by (p - 3) // 4.
pub fn chain_pm3div4(tmpvar1: &mut Fq, tmpvar0: &Fq) {
    *tmpvar1 = *tmpvar0;
    tmpvar1.square();
    //Self::sqr(tmpvar1, tmpvar0);                              /*    0 : 2 */
    let mut tmpvar9 = *tmpvar1;
    tmpvar9.mul_assign(tmpvar0);
    //Self::mul(&mut tmpvar9, tmpvar1, tmpvar0);                /*    1 : 3 */
    let mut tmpvar5 = *tmpvar1;
    tmpvar5.square();
    //Self::sqr(&mut tmpvar5, tmpvar1);                         /*    2 : 4 */
    let mut tmpvar2 = tmpvar9;
    tmpvar2.mul_assign(tmpvar1);
    //Self::mul(&mut tmpvar2, &tmpvar9, tmpvar1);               /*    3 : 5 */
    let mut tmpvar7 = tmpvar5;
    tmpvar7.mul_assign(&tmpvar9);
    //Self::mul(&mut tmpvar7, &tmpvar5, &tmpvar9);              /*    4 : 7 */
    let mut tmpvar10 = tmpvar2;
    tmpvar10.mul_assign(&tmpvar5);
    //Self::mul(&mut tmpvar10, &tmpvar2, &tmpvar5);             /*    5 : 9 */
    let mut tmpvar13 = tmpvar7;
    tmpvar13.mul_assign(&tmpvar5);
    //Self::mul(&mut tmpvar13, &tmpvar7, &tmpvar5);             /*    6 : 11 */
    let mut tmpvar4 = tmpvar10;
    tmpvar4.mul_assign(&tmpvar5);
    //Self::mul(&mut tmpvar4, &tmpvar10, &tmpvar5);             /*    7 : 13 */
    let mut tmpvar8 = tmpvar13;
    tmpvar8.mul_assign(&tmpvar5);
    //Self::mul(&mut tmpvar8, &tmpvar13, &tmpvar5);             /*    8 : 15 */
    let mut tmpvar15 = tmpvar4;
    tmpvar15.mul_assign(&tmpvar5);
    //Self::mul(&mut tmpvar15, &tmpvar4, &tmpvar5);             /*    9 : 17 */
    let mut tmpvar11 = tmpvar8;
    tmpvar11.mul_assign(&tmpvar5);
    //Self::mul(&mut tmpvar11, &tmpvar8, &tmpvar5);             /*   10 : 19 */
    let mut tmpvar3 = tmpvar15;
    tmpvar3.mul_assign(&tmpvar5);
    //Self::mul(&mut tmpvar3, &tmpvar15, &tmpvar5);             /*   11 : 21 */
    let mut tmpvar12 = tmpvar11;
    tmpvar12.mul_assign(&tmpvar5);
    //Self::mul(&mut tmpvar12, &tmpvar11, &tmpvar5);            /*   12 : 23 */
    *tmpvar1 = tmpvar4;
    tmpvar1.square();
    //Self::sqr(tmpvar1, &tmpvar4);                             /*   13 : 26 */
    let mut tmpvar14 = tmpvar12;
    tmpvar14.mul_assign(&tmpvar5);
    //Self::mul(&mut tmpvar14, &tmpvar12, &tmpvar5);            /*   14 : 27 */
    let mut tmpvar6 = *tmpvar1;
    tmpvar6.mul_assign(&tmpvar9);
    //Self::mul(&mut tmpvar6, tmpvar1, &tmpvar9);               /*   15 : 29 */
    let mut tmpvar5 = *tmpvar1;
    tmpvar5.mul_assign(&tmpvar2);
    //Self::mul(&mut tmpvar5, tmpvar1, &tmpvar2);               /*   16 : 31 */
    for _ in 0..12 {
        tmpvar1.square();
    } /*   17 : 106496 */
    tmpvar1.mul_assign(&tmpvar15); /*   29 : 106513 */
    for _ in 0..7 {
        tmpvar1.square();
    } /*   30 : 13633664 */
    tmpvar1.mul_assign(&tmpvar8); /*   37 : 13633679 */
    for _ in 0..4 {
        tmpvar1.square();
    } /*   38 : 218138864 */
    tmpvar1.mul_assign(&tmpvar2); /*   42 : 218138869 */
    for _ in 0..6 {
        tmpvar1.square();
    } /*   43 : 13960887616 */
    tmpvar1.mul_assign(&tmpvar7); /*   49 : 13960887623 */
    for _ in 0..7 {
        tmpvar1.square();
    } /*   50 : 1786993615744 */
    tmpvar1.mul_assign(&tmpvar12); /*   57 : 1786993615767 */
    for _ in 0..5 {
        tmpvar1.square();
    } /*   58 : 57183795704544 */
    tmpvar1.mul_assign(&tmpvar5); /*   63 : 57183795704575 */
    for _ in 0..2 {
        tmpvar1.square();
    } /*   64 : 228735182818300 */
    tmpvar1.mul_assign(&tmpvar9); /*   66 : 228735182818303 */
    for _ in 0..6 {
        tmpvar1.square();
    } /*   67 : 14639051700371392 */
    tmpvar1.mul_assign(&tmpvar4); /*   73 : 14639051700371405 */
    for _ in 0..6 {
        tmpvar1.square();
    } /*   74 : 936899308823769920 */
    tmpvar1.mul_assign(&tmpvar4); /*   80 : 936899308823769933 */
    for _ in 0..6 {
        tmpvar1.square();
    } /*   81 : 59961555764721275712 */
    tmpvar1.mul_assign(&tmpvar10); /*   87 : 59961555764721275721 */
    for _ in 0..3 {
        tmpvar1.square();
    } /*   88 : 479692446117770205768 */
    tmpvar1.mul_assign(&tmpvar9); /*   91 : 479692446117770205771 */
    for _ in 0..7 {
        tmpvar1.square();
    } /*   92 : 61400633103074586338688 */
    tmpvar1.mul_assign(&tmpvar4); /*   99 : 61400633103074586338701 */
    for _ in 0..4 {
        tmpvar1.square();
    } /*  100 : 982410129649193381419216 */
    tmpvar1.mul_assign(&tmpvar4); /*  104 : 982410129649193381419229 */
    for _ in 0..6 {
        tmpvar1.square();
    } /*  105 : 62874248297548376410830656 */
    tmpvar1.mul_assign(&tmpvar8); /*  111 : 62874248297548376410830671 */
    for _ in 0..6 {
        tmpvar1.square();
    } /*  112 : 4023951891043096090293162944 */
    tmpvar1.mul_assign(&tmpvar14); /*  118 : 4023951891043096090293162971 */
    for _ in 0..3 {
        tmpvar1.square();
    } /*  119 : 32191615128344768722345303768 */
    tmpvar1.mul_assign(tmpvar0); /*  122 : 32191615128344768722345303769 */
    for _ in 0..8 {
        tmpvar1.square();
    } /*  123 : 8241053472856260792920397764864 */
    tmpvar1.mul_assign(&tmpvar4); /*  131 : 8241053472856260792920397764877 */
    for _ in 0..7 {
        tmpvar1.square();
    } /*  132 : 1054854844525601381493810913904256 */
    tmpvar1.mul_assign(&tmpvar12); /*  139 : 1054854844525601381493810913904279 */
    for _ in 0..5 {
        tmpvar1.square();
    } /*  140 : 33755355024819244207801949244936928 */
    tmpvar1.mul_assign(&tmpvar13); /*  145 : 33755355024819244207801949244936939 */
    for _ in 0..6 {
        tmpvar1.square();
    } /*  146 : 2160342721588431629299324751675964096 */
    tmpvar1.mul_assign(&tmpvar4); /*  152 : 2160342721588431629299324751675964109 */
    for _ in 0..6 {
        tmpvar1.square();
    } /*  153 : 138261934181659624275156784107261702976 */
    tmpvar1.mul_assign(&tmpvar6); /*  159 : 138261934181659624275156784107261703005 */
    for _ in 0..4 {
        tmpvar1.square();
    } /*  160 : 2212190946906553988402508545716187248080 */
    tmpvar1.mul_assign(&tmpvar10); /*  164 : 2212190946906553988402508545716187248089 */
    for _ in 0..8 {
        tmpvar1.square();
    } /*  165 : 566320882408077821031042187703343935510784 */
    tmpvar1.mul_assign(&tmpvar6); /*  173 : 566320882408077821031042187703343935510813 */
    for _ in 0..4 {
        tmpvar1.square();
    } /*  174 : 9061134118529245136496675003253502968173008 */
    tmpvar1.mul_assign(&tmpvar4); /*  178 : 9061134118529245136496675003253502968173021 */
    for _ in 0..7 {
        tmpvar1.square();
    } /*  179 : 1159825167171743377471574400416448379926146688 */
    tmpvar1.mul_assign(&tmpvar12); /*  186 : 1159825167171743377471574400416448379926146711 */
    for _ in 0..9 {
        tmpvar1.square();
    } /*  187 : 593830485591932609265446093013221570522187116032 */
    tmpvar1.mul_assign(&tmpvar11); /*  196 : 593830485591932609265446093013221570522187116051 */
    for _ in 0..2 {
        tmpvar1.square();
    } /*  197 : 2375321942367730437061784372052886282088748464204 */
    tmpvar1.mul_assign(&tmpvar9); /*  199 : 2375321942367730437061784372052886282088748464207 */
    for _ in 0..5 {
        tmpvar1.square();
    } /*  200 : 76010302155767373985977099905692361026839950854624 */
    tmpvar1.mul_assign(&tmpvar7); /*  205 : 76010302155767373985977099905692361026839950854631 */
    for _ in 0..7 {
        tmpvar1.square();
    } /*  206 : 9729318675938223870205068787928622211435513709392768 */
    tmpvar1.mul_assign(&tmpvar2); /*  213 : 9729318675938223870205068787928622211435513709392773 */
    for _ in 0..7 {
        tmpvar1.square();
    } /*  214 : 1245352790520092655386248804854863643063745754802274944 */
    tmpvar1.mul_assign(&tmpvar10); /*  221 : 1245352790520092655386248804854863643063745754802274953 */
    for _ in 0..6 {
        tmpvar1.square();
    } /*  222 : 79702578593285929944719923510711273156079728307345596992 */
    tmpvar1.mul_assign(&tmpvar12); /*  228 : 79702578593285929944719923510711273156079728307345597015 */
    for _ in 0..5 {
        tmpvar1.square();
    } /*  229 : 2550482514985149758231037552342760740994551305835059104480 */
    tmpvar1.mul_assign(&tmpvar6); /*  234 : 2550482514985149758231037552342760740994551305835059104509 */
    for _ in 0..5 {
        tmpvar1.square();
    } /*  235 : 81615440479524792263393201674968343711825641786721891344288 */
    tmpvar1.mul_assign(&tmpvar11); /*  240 : 81615440479524792263393201674968343711825641786721891344307 */
    for _ in 0..5 {
        tmpvar1.square();
    } /*  241 : 2611694095344793352428582453598986998778420537175100523017824 */
    tmpvar1.mul_assign(&tmpvar11); /*  246 : 2611694095344793352428582453598986998778420537175100523017843 */
    for _ in 0..8 {
        tmpvar1.square();
    } /*  247 : 668593688408267098221717108121340671687275657516825733892567808 */
    tmpvar1.mul_assign(&tmpvar4); /*  255 : 668593688408267098221717108121340671687275657516825733892567821 */
    for _ in 0..7 {
        tmpvar1.square();
    } /*  256 : 85579992116258188572379789839531605975971284162153693938248681088 */
    tmpvar1.mul_assign(&tmpvar3); /*  263 : 85579992116258188572379789839531605975971284162153693938248681109 */
    for _ in 0..9 {
        tmpvar1.square();
    } /*  264 : 43816955963524192549058452397840182259697297491022691296383324727808 */
    tmpvar1.mul_assign(&tmpvar8); /*  273 : 43816955963524192549058452397840182259697297491022691296383324727823 */
    for _ in 0..5 {
        tmpvar1.square();
    } /*  274 : 1402142590832774161569870476730885832310313519712726121484266391290336 */
    tmpvar1.mul_assign(&tmpvar4); /*  279 : 1402142590832774161569870476730885832310313519712726121484266391290349 */
    for _ in 0..3 {
        tmpvar1.square();
    } /*  280 : 11217140726662193292558963813847086658482508157701808971874131130322792 */
    tmpvar1.mul_assign(&tmpvar9); /*  283 : 11217140726662193292558963813847086658482508157701808971874131130322795 */
    for _ in 0..8 {
        tmpvar1.square();
    } /*  284 : 2871588026025521482895094736344854184571522088371663096799777569362635520 */
    tmpvar1.mul_assign(&tmpvar8); /*  292 : 2871588026025521482895094736344854184571522088371663096799777569362635535 */
    for _ in 0..3 {
        tmpvar1.square();
    } /*  293 : 22972704208204171863160757890758833476572176706973304774398220554901084280 */
    tmpvar1.mul_assign(&tmpvar9); /*  296 : 22972704208204171863160757890758833476572176706973304774398220554901084283 */
    for _ in 0..7 {
        tmpvar1.square();
    } /*  297 : 2940506138650133998484577010017130685001238618492583011122972231027338788224 */
    tmpvar1.mul_assign(&tmpvar10); /*  304 : 2940506138650133998484577010017130685001238618492583011122972231027338788233 */
    for _ in 0..9 {
        tmpvar1.square();
    } /*  305 : 1505539142988868607224103429128770910720634172668202501694961782285997459575296 */
    tmpvar1.mul_assign(&tmpvar8); /*  314 : 1505539142988868607224103429128770910720634172668202501694961782285997459575311 */
    for _ in 0..6 {
        tmpvar1.square();
    } /*  315 : 96354505151287590862342619464241338286120587050764960108477554066303837412819904 */
    tmpvar1.mul_assign(&tmpvar3); /*  321 : 96354505151287590862342619464241338286120587050764960108477554066303837412819925 */
    for _ in 0..6 {
        tmpvar1.square();
    } /*  322 : 6166688329682405815189927645711445650311717571248957446942563460243445594420475200 */
    tmpvar1.mul_assign(&tmpvar5); /*  328 : 6166688329682405815189927645711445650311717571248957446942563460243445594420475231 */
    for _ in 0..5 {
        tmpvar1.square();
    } /*  329 : 197334026549836986086077684662766260809974962279966638302162030727790259021455207392 */
    tmpvar1.mul_assign(&tmpvar5); /*  334 : 197334026549836986086077684662766260809974962279966638302162030727790259021455207423 */
    for _ in 0..5 {
        tmpvar1.square();
    } /*  335 : 6314688849594783554754485909208520345919198792958932425669184983289288288686566637536 */
    tmpvar1.mul_assign(&tmpvar5); /*  340 : 6314688849594783554754485909208520345919198792958932425669184983289288288686566637567 */
    for _ in 0..4 {
        tmpvar1.square();
    } /*  341 : 101035021593516536876071774547336325534707180687342918810706959732628612618985066201072 */
    tmpvar1.mul_assign(&tmpvar4); /*  345 : 101035021593516536876071774547336325534707180687342918810706959732628612618985066201085 */
    for _ in 0..3 {
        tmpvar1.square();
    } /*  346 : 808280172748132295008574196378690604277657445498743350485655677861028900951880529608680 */
    tmpvar1.mul_assign(&tmpvar9); /*  349 : 808280172748132295008574196378690604277657445498743350485655677861028900951880529608683 */
    for _ in 0..8 {
        tmpvar1.square();
    } /*  350 : 206919724223521867522194994272944794695080306047678297724327853532423398643681415579822848 */
    tmpvar1.mul_assign(&tmpvar3); /*  358 : 206919724223521867522194994272944794695080306047678297724327853532423398643681415579822869 */
    for _ in 0..7 {
        tmpvar1.square();
    } /*  359 : 26485724700610799042840959266936933720970279174102822108713965252150195026391221194217327232 */
    tmpvar1.mul_assign(&tmpvar5); /*  366 : 26485724700610799042840959266936933720970279174102822108713965252150195026391221194217327263 */
    for _ in 0..5 {
        tmpvar1.square();
    } /*  367 : 847543190419545569370910696541981879071048933571290307478846888068806240844519078214954472416 */
    tmpvar1.mul_assign(&tmpvar5); /*  372 : 847543190419545569370910696541981879071048933571290307478846888068806240844519078214954472447 */
    for _ in 0..5 {
        tmpvar1.square();
    } /*  373 : 27121382093425458219869142289343420130273565874281289839323100418201799707024610502878543118304 */
    tmpvar1.mul_assign(&tmpvar5); /*  378 : 27121382093425458219869142289343420130273565874281289839323100418201799707024610502878543118335 */
    for _ in 0..4 {
        tmpvar1.square();
    } /*  379 : 433942113494807331517906276629494722084377053988500637429169606691228795312393768046056689893360 */
    tmpvar1.mul_assign(&tmpvar8); /*  383 : 433942113494807331517906276629494722084377053988500637429169606691228795312393768046056689893375 */
    for _ in 0..4 {
        tmpvar1.square();
    } /*  384 : 6943073815916917304286500426071915553350032863816010198866713707059660724998300288736907038294000 */
    tmpvar1.mul_assign(&tmpvar7); /*  388 : 6943073815916917304286500426071915553350032863816010198866713707059660724998300288736907038294007 */
    for _ in 0..7 {
        tmpvar1.square();
    } /*  389 : 888713448437365414948672054537205190828804206568449305454939354503636572799782436958324100901632896 */
    tmpvar1.mul_assign(&tmpvar5); /*  396 : 888713448437365414948672054537205190828804206568449305454939354503636572799782436958324100901632927 */
    for _ in 0..5 {
        tmpvar1.square();
    } /*  397 : 28438830349995693278357505745190566106521734610190377774558059344116370329593037982666371228852253664 */
    tmpvar1.mul_assign(&tmpvar6); /*  402 : 28438830349995693278357505745190566106521734610190377774558059344116370329593037982666371228852253693 */
    for _ in 0..5 {
        tmpvar1.square();
    } /*  403 : 910042571199862184907440183846098115408695507526092088785857899011723850546977215445323879323272118176 */
    tmpvar1.mul_assign(&tmpvar5); /*  408 : 910042571199862184907440183846098115408695507526092088785857899011723850546977215445323879323272118207 */
    for _ in 0..5 {
        tmpvar1.square();
    } /*  409 : 29121362278395589917038085883075139693078256240834946841147452768375163217503270894250364138344707782624 */
    tmpvar1.mul_assign(&tmpvar5); /*  414 : 29121362278395589917038085883075139693078256240834946841147452768375163217503270894250364138344707782655 */
    for _ in 0..5 {
        tmpvar1.square();
    } /*  415 : 931883592908658877345218748258404470178504199706718298916718488588005222960104668616011652427030649044960 */
    tmpvar1.mul_assign(&tmpvar5); /*  420 : 931883592908658877345218748258404470178504199706718298916718488588005222960104668616011652427030649044991 */
    for _ in 0..5 {
        tmpvar1.square();
    } /*  421 : 29820274973077084075046999944268943045712134390614985565334991634816167134723349395712372877664980769439712 */
    tmpvar1.mul_assign(&tmpvar5); /*  426 : 29820274973077084075046999944268943045712134390614985565334991634816167134723349395712372877664980769439743 */
    for _ in 0..5 {
        tmpvar1.square();
    } /*  427 : 954248799138466690401503998216606177462788300499679538090719732314117348311147180662795932085279384622071776 */
    tmpvar1.mul_assign(&tmpvar5); /*  432 : 954248799138466690401503998216606177462788300499679538090719732314117348311147180662795932085279384622071807 */
    for _ in 0..5 {
        tmpvar1.square();
    } /*  433 : 30535961572430934092848127942931397678809225615989745218903031434051755145956709781209469826728940307906297824 */
    tmpvar1.mul_assign(&tmpvar5); /*  438 : 30535961572430934092848127942931397678809225615989745218903031434051755145956709781209469826728940307906297855 */
    for _ in 0..4 {
        tmpvar1.square();
    } /*  439 : 488575385158894945485570047086902362860947609855835923502448502944828082335307356499351517227663044926500765680 */
    tmpvar1.mul_assign(&tmpvar4); /*  443 : 488575385158894945485570047086902362860947609855835923502448502944828082335307356499351517227663044926500765693 */
    for _ in 0..6 {
        tmpvar1.square();
    } /*  444 : 31268824650169276511076483013561751223100647030773499104156704188468997269459670815958497102570434875296049004352 */
    tmpvar1.mul_assign(&tmpvar3); /*  450 : 31268824650169276511076483013561751223100647030773499104156704188468997269459670815958497102570434875296049004373 */
    for _ in 0..4 {
        tmpvar1.square();
    } /*  451 : 500301194402708424177223728216988019569610352492375985666507267015503956311354733055335953641126958004736784069968 */
    tmpvar1.mul_assign(&tmpvar2); /*  455 : 500301194402708424177223728216988019569610352492375985666507267015503956311354733055335953641126958004736784069973 */
    tmpvar1.square(); /*  456 : 1000602388805416848354447456433976039139220704984751971333014534031007912622709466110671907282253916009473568139946 */
}

#[test]
fn test_fq_chain() {
    use rand::{thread_rng, Rand};

    let mut rng = thread_rng();
    let p_m3_over4 = [
        0xee7fbfffffffeaaau64,
        0x7aaffffac54ffffu64,
        0xd9cc34a83dac3d89u64,
        0xd91dd2e13ce144afu64,
        0x92c6e9ed90d2eb35u64,
        0x680447a8e5ff9a6u64,
    ];

    let mut result = Fq::zero();
    for _ in 0..32 {
        let mut input = Fq::rand(&mut rng);
        chain_pm3div4(&mut result, &input);
        input = input.pow(&p_m3_over4);
        assert_eq!(input, result);
    }
}
