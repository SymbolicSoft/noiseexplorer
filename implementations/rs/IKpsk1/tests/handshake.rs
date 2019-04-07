#![allow(non_snake_case, non_upper_case_globals)]

use noiseexplorer_ikpsk1::{
	noisesession::NoiseSession,
	types::{Keypair, Message, MessageBuffer, PrivateKey, Psk, PublicKey},
};

#[test]
fn noiseexplorer_test_ikpsk1() {
    let prologueA: Message = Message::from_str("4a6f686e2047616c74");
	let prologueB: Message = Message::from_str("4a6f686e2047616c74");
	let initStaticA: PrivateKey = PrivateKey::from_str("e61ef9919cde45dd5f82166404bd08e38bceb5dfdfded0a34c8df7ed542214d1");
	let respStatic_private: PrivateKey = PrivateKey::from_str("4a3acbfdb163dec651dfa3194dece676d437029c62a408b4c5ea9114246e4893");
	let respStatic_public: PublicKey = PrivateKey::from_str("4a3acbfdb163dec651dfa3194dece676d437029c62a408b4c5ea9114246e4893").generate_public_key();
	let pskA: Psk = Psk::from_str("54686973206973206d7920417573747269616e20706572737065637469766521");
	let pskB: Psk = Psk::from_str("54686973206973206d7920417573747269616e20706572737065637469766521");
	let mut initiatorSession: NoiseSession = NoiseSession::init_session(true, prologueA, Keypair::from_private_key(initStaticA), respStatic_public, pskA);
	let mut responderSession: NoiseSession = NoiseSession::init_session(false, prologueB, Keypair::from_private_key(respStatic_private), PublicKey::empty(), pskB);
	initiatorSession.set_ephemeral_keypair(Keypair::from_private_key(PrivateKey::from_str("893e28b9dc6ca8d611ab664754b8ceb7bac5117349a4439a6b0569da977c464a")));
	responderSession.set_ephemeral_keypair(Keypair::from_private_key(PrivateKey::from_str("bbdb4cdbd309f1a1f2e1456967fe288cadd6f712d65dc7b7793d5e63da6b375b")));
	let mut messageA: MessageBuffer = initiatorSession.send_message(Message::from_str("4c756477696720766f6e204d69736573"));
	let mut validA: bool = false;
	if let Some(_x) = responderSession.recv_message(&mut messageA) {
		validA = true;
	}
	let tA: Message = Message::from_str("ca35def5ae56cec33dc2036731ab14896bc4c75dbb07a61f879f8e3afa4c794498e192a0a94102bd8fa1a182979c012f4fa2558d899e2e58d4d4aba041a56b35297560de33bf7fe93f8e567791039539f59e76a00721ea7c1095fbccf10a13df79f3b5605bfb0617c309698737c73429");
	let mut messageB: MessageBuffer = responderSession.send_message(Message::from_str("4d757272617920526f746862617264"));
	let mut validB: bool = false;
	if let Some(_x) = initiatorSession.recv_message(&mut messageB) {
		validB = true;
	}
	let tB: Message = Message::from_str("95ebc60d2b1fa672c1f46a8aa265ef51bfe38e7ccb39ec5be34069f1448088434523a21bc9f1ce57af3dc28365e1e33c25f577fc4aa2149d5d6a2ab0911beb");
	let mut messageC: MessageBuffer = initiatorSession.send_message(Message::from_str("462e20412e20486179656b"));
	let mut validC: bool = false;
	if let Some(_x) = responderSession.recv_message(&mut messageC) {
		validC = true;
	}
	let tC: Message = Message::from_str("dc15d1ceff592ff648bba38f9bc63c0049600307fba700ba2a0b2b");
	let mut messageD: MessageBuffer = responderSession.send_message(Message::from_str("4361726c204d656e676572"));
	let mut validD: bool = false;
	if let Some(_x) = initiatorSession.recv_message(&mut messageD) {
		validD = true;
	}
	let tD: Message = Message::from_str("85f1e8c573c0d9fd188080532a0ad1a6d457974c91f2ff0f21ecaf");
	let mut messageE: MessageBuffer = initiatorSession.send_message(Message::from_str("4a65616e2d426170746973746520536179"));
	let mut validE: bool = false;
	if let Some(_x) = responderSession.recv_message(&mut messageE) {
		validE = true;
	}
	let tE: Message = Message::from_str("11d83f8ff550ef18c1314540ade9c7b9e5fb5245889221856ea55b0b8e64bdf1bc");
	let mut messageF: MessageBuffer = responderSession.send_message(Message::from_str("457567656e2042f6686d20766f6e2042617765726b"));
	let mut validF: bool = false;
	if let Some(_x) = initiatorSession.recv_message(&mut messageF) {
		validF = true;
	}
	let tF: Message = Message::from_str("b7b3a985fe737290fb597224ccad3f9ad3caa3d396bf201233891db26172d267f4298d47c2");
	assert!(
		validA && validB && validC && validD && validE && validF,
		"Sanity check FAIL for IKpsk1_25519_ChaChaPoly_BLAKE2s."
	);
	let mut cA: Vec<u8> = Vec::new();
	cA.append(&mut Vec::from(&messageA.ne[..]));
	cA.append(&mut messageA.ns);
	cA.append(&mut messageA.ciphertext);
	let mut cB: Vec<u8> = Vec::new();
	cB.append(&mut Vec::from(&messageB.ne[..]));
	cB.append(&mut messageB.ciphertext);
	let mut cC: Vec<u8> = Vec::new();
	cC.append(&mut messageC.ciphertext);
	let mut cD: Vec<u8> = Vec::new();
	cD.append(&mut messageD.ciphertext);
	let mut cE: Vec<u8> = Vec::new();
	cE.append(&mut messageE.ciphertext);
	let mut cF: Vec<u8> = Vec::new();
	cF.append(&mut messageF.ciphertext);
	assert!(tA.as_bytes() == &cA,
		"\n\n\nTest A: FAIL\n\nExpected:\n{:X?}\n\nActual:\n{:X?}\n\n\n",
		tA.as_bytes(),
		&cB
	);
	assert!(tB.as_bytes() == &cB,
		"\n\n\nTest B: FAIL\n\nExpected:\n{:X?}\n\nActual:\n{:X?}\n\n\n",
		tB.as_bytes(),
		&cB
	);
	assert!(tC.as_bytes() == &cC,
		"\n\n\nTest C: FAIL\n\nExpected:\n{:X?}\n\nActual:\n{:X?}\n\n\n",
		tC.as_bytes(),
		&cB
	);
	assert!(tD.as_bytes() == &cD,
		"\n\n\nTest D: FAIL\n\nExpected:\n{:X?}\n\nActual:\n{:X?}\n\n\n",
		tD.as_bytes(),
		&cB
	);
	assert!(tE.as_bytes() == &cE,
		"\n\n\nTest E: FAIL\n\nExpected:\n{:X?}\n\nActual:\n{:X?}\n\n\n",
		tE.as_bytes(),
		&cB
	);
	assert!(tF.as_bytes() == &cF,
		"\n\n\nTest F: FAIL\n\nExpected:\n{:X?}\n\nActual:\n{:X?}\n\n\n",
		tF.as_bytes(),
		&cB
	);
}
