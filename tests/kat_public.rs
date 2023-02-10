mod public_memory_stores;

use std::num::ParseIntError;

use blind_rsa_signatures::{KeyPair, Options, PublicKey, SecretKey};

use public_memory_stores::*;
use rand::{CryptoRng, Error, RngCore};
use tls_codec::Serialize;

use privacypass::{
    auth::authenticate::TokenChallenge,
    public_tokens::{client::*, public_key_to_token_key_id, server::*},
};

const SKS: &str = "2d2d2d2d2d424547494e2050524956415445204b45592d2d2d2d2d0a4d494945765149424144414e42676b71686b6947397730424151454641415343424b63776767536a41674541416f49424151444c4775317261705831736334420a4f6b7a38717957355379356b6f6a41303543554b66717444774e38366a424b5a4f76457245526b49314c527876734d6453327961326333616b4745714c756b440a556a35743561496b3172417643655844644e44503442325055707851436e6969396e6b492b6d67725769744444494871386139793137586e6c5079596f784f530a646f6558563835464f314a752b62397336356d586d34516a7551394559614971383371724450567a50335758712b524e4d636379323269686763624c766d42390a6a41355334475666325a6c74785954736f4c364872377a58696a4e39463748627165676f753967654b524d584645352f2b4a3956595a634a734a624c756570480a544f72535a4d4948502b5358514d4166414f454a4547426d6d4430683566672f43473475676a79486e4e51383733414e4b6a55716d3676574574413872514c620a4530742b496c706641674d4241414543676745414c7a4362647a69316a506435384d6b562b434c6679665351322b7266486e7266724665502f566344787275690a3270316153584a596962653645532b4d622f4d4655646c485067414c773178513457657266366336444373686c6c784c57535638477342737663386f364750320a6359366f777042447763626168474b556b5030456b62395330584c4a57634753473561556e484a585237696e7834635a6c666f4c6e7245516536685578734d710a6230644878644844424d644766565777674b6f6a4f6a70532f39386d4555793756422f3661326c7265676c766a632f326e4b434b7459373744376454716c47460a787a414261577538364d435a342f5131334c762b426566627174493973715a5a776a7264556851483856437872793251564d515751696e57684174364d7154340a53425354726f6c5a7a7772716a65384d504a393175614e4d6458474c63484c49323673587a76374b53514b42675144766377735055557641395a325a583958350a6d49784d54424e6445467a56625550754b4b413179576e31554d444e63556a71682b7a652f376b337946786b68305146333162713630654c393047495369414f0a354b4f574d39454b6f2b7841513262614b314d664f5931472b386a7a42585570427339346b353353383879586d4b366e796467763730424a385a6835666b55710a5732306f5362686b686a5264537a48326b52476972672b5553774b426751445a4a4d6e7279324578612f3345713750626f737841504d69596e6b354a415053470a79327a305a375455622b7548514f2f2b78504d376e433075794c494d44396c61544d48776e3673372f4c62476f455031575267706f59482f4231346b2f526e360a667577524e3632496f397463392b41434c745542377674476179332b675277597453433262356564386c4969656774546b6561306830754453527841745673330a6e356b796132513976514b4267464a75467a4f5a742b7467596e576e51554567573850304f494a45484d45345554644f637743784b7248527239334a6a7546320a453377644b6f546969375072774f59496f614a5468706a50634a62626462664b792b6e735170315947763977644a724d6156774a6376497077563676315570660a56744c61646d316c6b6c7670717336474e4d386a6e4d30587833616a6d6d6e66655739794758453570684d727a4c4a6c394630396349324c416f4742414e58760a75675658727032627354316f6b6436755361427367704a6a5065774e526433635a4b397a306153503144544131504e6b7065517748672f2b36665361564f487a0a79417844733968355272627852614e6673542b7241554837783153594456565159564d68555262546f5a6536472f6a716e544333664e6648563178745a666f740a306c6f4d4867776570362b53494d436f6565325a6374755a5633326c63496166397262484f633764416f47416551386b3853494c4e4736444f413331544535500a6d3031414a49597737416c5233756f2f524e61432b78596450553354736b75414c78786944522f57734c455142436a6b46576d6d4a41576e51554474626e594e0a536377523847324a36466e72454374627479733733574156476f6f465a6e636d504c50386c784c79626c534244454c79615a762f624173506c4d4f39624435630a4a2b4e534261612b6f694c6c31776d4361354d43666c633d0a2d2d2d2d2d454e442050524956415445204b45592d2d2d2d2d0a";
const PKS: &str = "30820152303d06092a864886f70d01010a3030a00d300b0609608648016503040202a11a301806092a864886f70d010108300b0609608648016503040202a2030201300382010f003082010a0282010100cb1aed6b6a95f5b1ce013a4cfcab25b94b2e64a23034e4250a7eab43c0df3a8c12993af12b111908d4b471bec31d4b6c9ad9cdda90612a2ee903523e6de5a224d6b02f09e5c374d0cfe01d8f529c500a78a2f67908fa682b5a2b430c81eaf1af72d7b5e794fc98a3139276879757ce453b526ef9bf6ceb99979b8423b90f4461a22af37aab0cf5733f7597abe44d31c732db68a181c6cbbe607d8c0e52e0655fd9996dc584eca0be87afbcd78a337d17b1dba9e828bbd81e291317144e7ff89f55619709b096cbb9ea474cead264c2073fe49740c01f00e109106066983d21e5f83f086e2e823c879cd43cef700d2a352a9babd612d03cad02db134b7e225a5f0203010001";
const CHALLENGE: &str = "0002000e6973737565722e6578616d706c6500000e6f726967696e2e6578616d706c65";
const NONCE: &str = "9f146902790e692bd689253239fd2a1cdd816815916df1a39aaebfb6f41e71f4";
const BLIND: &str ="128b3749582beb7680bd7388b4c7f05b85998e0f6fdcadd9a2803c2be38513c7acf5e12c160111ccd01bfc66016d6d742daf4e639c35e9cd9512834685999f0b148dcd1a9cbda9a7d0d85da041547e20477887960df66fc95836b6535edc019aaa33f547056fe2553c200dc966a8ddea0c4394f78f275a21464f003036573f6277206ecd933f4e2abac330a5285879d4a93b543a79bf197219fdee4963628025c17bc83fc2321237fcdebaefdc132d69dd2938396f469d55dcb2d73d7eb3a7c0ccfd1ebb49ca5629b7a48e56394b5958cec3500be2617e1ce7618a44e1f9a0140fb1658bbb2aa7a346bdac07defecac5db1e10257bbd90d1b98671c446b37194";
const SALT: &str = "6e7e18003889833c4e6abf4cc9d8dbbc8114190a3c675c4fd5cc1a60c2832839f90c4019321067d2dc0b2c1d3dbdb273";
const TOKEN_REQUEST: &str = "000208945cc74d3f86d2f2c71b4ee8c2bc75605cf517ffae0021b34a98dbe6595554cf8c6287ce9ab323335d075383b271cdc761e62dc6df356cb3b7a4af3a25242c86827fc2c4bed4371bbddbda9ef56a1a98d6ba98eedd0f890fe3c564644bd7bf48ec906946e94e0395e3677297799b51ba850f7ee75b4086f1853cd9fdc507034fbdef1c097251e1dbc8bc318b1d489ae80882563b088a50d88b2f17398f6497f8ebb9062d38cc848a86fd5855fec83beff69c5ca089580b74046267e3029ac892cad833da2792c1858710883380f4763d59090c820e6cfcd5cf8e428fbd71330f35bc20ddfa8dbd0e035cba66e0e593a65ca2f3dd510fd630a99174ddcd6fa9ae";
const TOKEN_RESPONSE: &str = "13aa0e7842f39dc22c841e4d2655de7f7029dd4ecb80e2285ff42f35889d76d13a381ffa19385f3ba4c43456a5ef3522026c0932ef15d234f8e1d79287749865afdf9493ef8fe6fbd835173474fb51c106870faec90879cca156e5f4b795328a47e5967e37deaead700823bf49234310deb86f91a520ea477f6e51cc0c98df0164c72754bd91b36c50f6e0749241ee1fc9b2d763fd75ea1328db7b48e5021dbc723d4f708b6bd68d98a19aee9041dfd042af342cd6d07e77a968e93f4571685abb1155b7def325816655f524e199d973a1d0d02838dfbbc05ee5cf15694a45d8a6c6fccae3d7542eff876c510960de6bacb401fca9de049dbfc9dad92df8d70a";
const TOKEN: &str = "00029f146902790e692bd689253239fd2a1cdd816815916df1a39aaebfb6f41e71f411e15c91a7c2ad02abd66645802373db1d823bea80f08d452541fb2b62b5898bca572f8982a9ca248a3056186322d93ca147266121ddeb5632c07f1f71cd270874f7405fdc3e8c6bf2bbfa516fffbcb8f1caeecd295fe4932f9ea84b3c24935085c6039e6f15450252f4d65510e5daeb3ab43ae6ef79bf46a86c98106fbe38e6865d9355ecf116bd80716223da06168786d886c6e3f29ae966627dcc1138e7d5ce0e991c37f77828df4accb03ec4c9822433cfc21398178ffdc2400a0362fb258f565de8408ce860caa07393252064c63fb03c040465bb9eed0fde3cfda9fa764ae7aadffacab31c1266b8596c10b868ba9744f9c794c402732e4274ef0edd72dd9cbc2ba7aa41c92af08a25601ee3221e20730bf722cd768094c2faf259abaea811e9fcd6e8f1899686dda429bbd0af339983aadc614a63c83e0344345a9ccd";

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[tokio::test]
async fn kat_public_token() {
    // KAT: Decode hex strings
    let sks = decode_hex(SKS).unwrap();
    let pks = decode_hex(PKS).unwrap();
    let challenge = decode_hex(CHALLENGE).unwrap();
    let nonce = decode_hex(NONCE).unwrap();
    let mut blind = decode_hex(BLIND).unwrap();
    let salt = decode_hex(SALT).unwrap();
    let expected_token_request = decode_hex(TOKEN_REQUEST).unwrap();
    let expected_token_response = decode_hex(TOKEN_RESPONSE).unwrap();
    let expected_token = decode_hex(TOKEN).unwrap();

    // Server: Instantiate in-memory keystore and nonce store.
    let issuer_key_store = IssuerMemoryKeyStore::default();
    let origin_key_store = OriginMemoryKeyStore::default();
    let nonce_store = MemoryNonceStore::default();

    // Server: Create servers for issuer and origin
    let issuer_server = IssuerServer::new();
    let origin_server = OriginServer::new();

    // Keys
    let options = Options::default();

    let sec_key = SecretKey::from_pem(&String::from_utf8_lossy(&sks)).unwrap();
    let pub_key = PublicKey::from_spki(&pks, Some(&options)).unwrap();

    // KAT: Check public key
    // Derive the public key from the private and compare it
    assert_eq!(sec_key.to_public_key(), pub_key.0);

    // Serialize the public key and compare it
    assert_eq!(serialize_public_key(&pub_key), pks);

    let keypair = KeyPair {
        sk: sec_key,
        pk: pub_key.clone(),
    };

    // Issuer server: Set the keypair
    issuer_server.set_keypair(&issuer_key_store, keypair).await;

    // Origin key store: Set the public key
    origin_key_store
        .insert(public_key_to_token_key_id(&pub_key), pub_key.clone())
        .await;

    // Client: Create client
    let mut client = Client::new(pub_key);

    // Prepare the deterministic number generator
    blind.reverse();

    let det_rng = &mut DeterministicRng::new(nonce, salt, blind);

    let token_challenge = TokenChallenge::deserialize(challenge.as_slice()).unwrap();
    let challenge_digest: [u8; 32] = token_challenge.digest().unwrap();

    let (token_request, token_state) = client
        .issue_token_request(det_rng, token_challenge)
        .unwrap();

    // KAT: Check token request
    assert_eq!(
        token_request.tls_serialize_detached().unwrap(),
        expected_token_request
    );

    // Issuer server: Issue a TokenResponse
    let token_response = issuer_server
        .issue_token_response(&issuer_key_store, token_request)
        .await
        .unwrap();

    // KAT: Check token response
    assert_eq!(
        token_response.tls_serialize_detached().unwrap(),
        expected_token_response
    );

    // Client: Turn the TokenResponse into a Token
    let token = client.issue_token(token_response, &token_state).unwrap();

    // Compare the challenge digest
    assert_eq!(token.challenge_digest(), &challenge_digest);

    // Origin server: Redeem the token
    assert!(origin_server
        .redeem_token(&origin_key_store, &nonce_store, token.clone())
        .await
        .is_ok());

    // KAT: Check token
    assert_eq!(token.tls_serialize_detached().unwrap(), expected_token);
}

// Helper RNG that returns the same set of values for each call to (try_)fill_bytes.

enum RngStep {
    Nonce,
    Blind,
    Salt,
}

struct DeterministicRng {
    nonce: Vec<u8>,
    salt: Vec<u8>,
    blind: Vec<u8>,
    step: RngStep,
}

impl DeterministicRng {
    #[cfg(test)]
    fn new(nonce: Vec<u8>, salt: Vec<u8>, blind: Vec<u8>) -> Self {
        Self {
            nonce,
            salt,
            blind,
            step: RngStep::Nonce,
        }
    }

    fn fill_with_data(&mut self, dest: &mut [u8]) {
        match self.step {
            RngStep::Nonce => {
                dest.copy_from_slice(&self.nonce);
                self.step = RngStep::Salt;
            }
            RngStep::Salt => {
                dest.copy_from_slice(&self.salt);
                self.step = RngStep::Blind;
            }
            RngStep::Blind => {
                dest.copy_from_slice(&self.blind);
                self.step = RngStep::Nonce;
            }
        }
    }
}

impl RngCore for DeterministicRng {
    fn next_u32(&mut self) -> u32 {
        unimplemented!()
    }

    fn next_u64(&mut self) -> u64 {
        unimplemented!()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.fill_with_data(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_with_data(dest);
        Ok(())
    }
}

impl CryptoRng for DeterministicRng {}
