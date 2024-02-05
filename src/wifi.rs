// use esp_wifi::wifi::{WifiController, WifiDevice, WifiMode, WifiState, WifiEvent};
// use esp_wifi::{initialize, EspWifiInitFor};

// use embassy_net::{Config, Ipv4Address, Stack, StackResources, ConfigV4, StaticConfigV4, Ipv4Cidr, IpListenEndpoint};
// use embassy_net::tcp::TcpSocket;
// use embassy_time::{Duration, Timer};

// use embedded_svc::wifi::{ClientConfiguration, Configuration, Wifi, AuthMethod};

// use hal::clock::Clocks;
// // use hal::peripherals::TIMG0;
// use hal::system::RadioClockControl;
// use hal::systimer::{Alarm, Target};
// // use hal::timer::{Timer as HalTimer, Timer0};
// use hal::rng::Rng;
// use hal::radio::Wifi as HalWifi;
// // use hal::embassy;

// use esp_println::println;

// use crate::utility::singleton;

// const SSID: &str = "MochiMochiPoint";
// const PASSWORD: &str = "RamuneSattyuzai375";
// pub const LOCAL_ENDPOINT : Ipv4Cidr = Ipv4Cidr::new(Ipv4Address::new(192, 168, 137, 3), 24);

// /// ペリフェラルなど -[esp-wifi]-> WifiDevice, WifiController
// /// WifiDevice -[embassy-net] -> Stack
// /// で、StackとWifiControllerを返す
// pub fn make_device_and_controller<'a, 'b>(
// 	system_radio_clock_control: RadioClockControl,
// 	alarm0: Alarm<Target, 0>,
// 	// timer0: HalTimer<Timer0<TIMG0>>,
// 	rng: Rng,
// 	wifi: HalWifi,
// 	clocks: &Clocks,
// ) -> (&'static Stack<WifiDevice<'b>>, WifiController<'b>) {
// 	// まずはesp-wifiやespc3-halを使っていく
// 	// すなわち、物理層、データリンク層の初期化を行う。すなわちアクセスポイントとの接続までを行う。
// 	// これによってWifiDeviceとWifiControllerが作られる

// 	// なんのための引数か謎い
// 	let init = initialize(
// 		EspWifiInitFor::Wifi,
// 		alarm0,
// 		rng,
// 		system_radio_clock_control,
// 		&clocks,
// 	)
// 	.unwrap();

// 	let (wifi_interface, controller) =
// 		esp_wifi::wifi::new_with_mode(&init, wifi, WifiMode::Sta).unwrap(); // STAとして使う

// 	// ここからは、embassy-netを用いて
// 	// 物理層上で動くネットワークスタック(通信モデルの各層を受け持つようなオブジェクトを紐づけていくためのもの)を作る。

// 	// DHCPでIPアドレスを取得するための設定
// 	let config = Config {
// 		ipv4: ConfigV4::Static(StaticConfigV4 {
// 			address: LOCAL_ENDPOINT,
// 			gateway: None,
// 			dns_servers: Default::default(),
// 		}),
// 	};

// 	// 謎いシード。これは何？
// 	let seed = 1234; // very random, very secure seed

// 	// Init network stack
// 	let stack = &*singleton!(Stack::new(
// 		wifi_interface,
// 		config,
// 		singleton!(StackResources::<3>::new()),
// 		seed
// 	));

// 	(stack, controller)
// }


// // wifiの接続タスク
// #[embassy_executor::task]
// pub async fn wifi_connection(mut controller: WifiController<'static>) {
// 	println!("start connection task");
// 	println!("Device capabilities: {:?}", controller.get_capabilities());
// 	loop {
// 		match esp_wifi::wifi::get_wifi_state() {
// 			WifiState::StaConnected => {
// 				// wait until we're no longer connected
// 				controller.wait_for_event(WifiEvent::StaDisconnected).await;
// 				Timer::after(Duration::from_millis(5000)).await
// 			}
// 			_ => {}
// 		}
// 		if !matches!(controller.is_started(), Ok(true)) {
// 			let client_config = Configuration::Client(ClientConfiguration {
// 				ssid: SSID.into(),
// 				password: PASSWORD.into(),
// 				auth_method: AuthMethod::WPA2Personal,
// 				..Default::default()
// 			});
// 			controller.set_configuration(&client_config).unwrap();
// 			println!("Starting wifi");
// 			controller.start().await.unwrap();
// 			println!("Wifi started!");
// 		}
// 		println!("About to connect...");

// 		match controller.connect().await {
// 			Ok(_) => {
// 				println!("Wifi connected!");
// 			}
// 			Err(e) => {
// 				println!("Failed to connect to wifi: {e:?}");
// 				Timer::after(Duration::from_millis(5000)).await
// 			}
// 		}
// 	}
// }

// // ネットワークスタックのタスク。たぶんWifiの上にのっかってるやつを動かすためのバックグラウンドタスク
// #[embassy_executor::task]
// pub async fn net_task(stack: &'static Stack<WifiDevice<'static>>) {
// 	stack.run().await
// }


// // TCP通信を行うタスク
// #[embassy_executor::task]
// pub async fn tcp_task(stack: &'static Stack<WifiDevice<'static>>, buf: *mut AtomicU32) -> ! {
// 	let mut rx_buffer = [0; 4096];
// 	let mut tx_buffer = [0; 4096];

// 	loop {
// 		if stack.is_link_up() {
// 			break;
// 		}
// 		Timer::after(Duration::from_millis(500)).await;
// 	}

// 	println!("Waiting to get IP address...");
// 	println!("Hah! I have static IP address! :D");

// 	loop {
// 		Timer::after(Duration::from_millis(1_000)).await;

// 		let mut socket = TcpSocket::new(&stack, &mut rx_buffer, &mut tx_buffer);

// 		socket.set_timeout(Some(embassy_time::Duration::from_secs(10)));

// 		println!("connecting...");
// 		println!("Wait for connection...");
// 		let r = socket
// 			.accept(IpListenEndpoint {
// 				addr: None,
// 				port: 50000,
// 			})
// 			.await;
// 		println!("Connected...");

// 		if let Err(e) = r {
// 			println!("connect error: {:?}", e);
// 			continue;
// 		}

// 		println!("connected!");
// 		println!("I'll say it again. I've got a successful comms connection! XD");


// 		// [0] is_enabled
// 			// 0: disabled, 1: enabled
// 		// [1] left motor duty_pct
// 			// [-100, 100]
// 		// [2] right motor duty_pct
// 			// [-100, 100]
// 		loop {
// 			let mut tmp_buf = [0u8; 3];
// 			'try_read: loop {
// 				use embedded_io::asynch::Read;
// 				match socket.read_exact(&mut tmp_buf).await {
// 					Ok(()) => {
// 						println!("read ok! {:?}", &tmp_buf);
// 						break 'try_read
// 					},
// 					Err(e) => {
// 						println!("read error: {:?}", e);
// 						continue 'try_read;
// 					}
// 				};
// 			}
// 			let tmp_buf: u32 = tmp_buf[0] as u32 | (tmp_buf[1] as u32) << 8 | (tmp_buf[2] as u32) << 16;
// 			unsafe{(*buf).store(tmp_buf, Ordering::Relaxed);}
// 		}

// 		// Timer::after(Duration::from_millis(3000)).await;
// 	}
// }