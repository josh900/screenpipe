
You said:
readme:
<p align="center">
   <a href ="https://screenpi.pe">
      <img src="https://github.com/user-attachments/assets/d3b1de26-c3c0-4c84-b9c4-b03213b97a30" alt="logo" width="200">
   </a>
</p>

<pre align="center">
   ___  ___ _ __ ___  ___ _ __  _ __ (_)_ __   ___ 
  / __|/ __| '__/ _ \/ _ \ '_ \| '_ \| | '_ \ / _ \
  \__ \ (__| | |  __/  __/ | | | |_) | | |_) |  __/
  |___/\___|_|  \___|\___|_| |_| .__/|_| .__/ \___|
                               |_|     |_|         
</pre>

<p align="center">
    <a href="https://screenpi.pe" target="_blank">
        <img src="https://img.shields.io/badge/Download%20The-Desktop%20App-blue?style=for-the-badge" alt="Download the Desktop App">
    </a>
</p>

<p align="center">
    <a href="https://www.youtube.com/@mediar_ai" target="_blank">
        <img src="https://img.shields.io/endpoint?style=for-the-badge&url=https%3A%2F%2Fyoutube-channel-badge.ngoldack.vercel.app%2Fapi%2Fsubscriber" alt="Subs">
    </a>
</p>


<p align="center">
    <a href="https://discord.gg/dU9EBuw7Uq">
        <img src="https://img.shields.io/discord/823813159592001537?color=5865F2&logo=discord&logoColor=white&style=flat-square" alt="Join us on Discord">
    </a>
        <a href="https://twitter.com/screen_pipe"><img alt="X account" src="https://img.shields.io/twitter/url/https/twitter.com/diffuserslib.svg?style=social&label=Follow%20%40screen_pipe"></a>
</p>

<p align="center">
  <a href ="https://cal.com/louis030195/screenpipe">
    <img alt="Let's chat" src="https://cal.com/book-with-cal-dark.svg" />
  </a>
    


<p align="center">
   <a href ="https://screenpi.pe">
       <img alt="demo" src="https://github.com/user-attachments/assets/39d27adc-e17e-4ca5-89c5-faf45a3ea20f" width="800" />
   </a>
</p>

---

*Latest News* 🔥
- [2024/08] Anyone can now [create, share, install pipes](https://youtu.be/iCqHgZgQHyA?si=DjKJir7HfZoQKItK) (plugins) from the app interface based on a github repo/dir
- [2024/08] We're running bounties! Contribute to screenpipe & make money, [check issues](https://github.com/mediar-ai/screenpipe/issues)
- [2024/08] Audio input & output now works perfect on Windows, Linux, MacOS (<15.0). We also support multi monitor capture and defaulting STT to Whisper Distil large v3
- [2024/08] We released video embedding. AI gives you links to your video recording in the chat!
- [2024/08] We released the pipe store! Create, share, use plugins that get you the most out of your data in less than 30s, even if you are not technical.
- [2024/08] We released Apple & Windows Native OCR.
- [2024/08] **The Linux desktop app is here!**.
- [2024/07] **The Windows desktop app is here! [Get it now!](https://screenpi.pe)**.
- [2024/07] 🎁 Screenpipe won Friends (the AI necklace) hackathon at AGI House (integrations soon)
- [2024/07] **We just launched the desktop app! [Download now!](https://screenpi.pe)**

---

# 24/7 Screen & Audio Capture

Library to build personalized AI powered by what you've seen, said, or heard. Works with Ollama. Alternative to Rewind.ai. Open. Secure. You own your data. Rust.  
We are shipping daily, make suggestions, post bugs, [give feedback](mailto:louis@screenpi.pe?subject=Screenpipe%20Feedback&body=I'd%20like%20to%20use%20Screenpipe%20for%20...%0D%0A%0D%0AI%20cannot%20because%20of%20...%0D%0A%0D%0AWe%20can%20also%20have%20a%20call,%20book%20at%20https://cal.com/louis030195/screenpipe).

![diagram](./content/diagram2.png)

# Why?

Building a reliable stream of audio and screenshot data, where a user simply clicks a button and the script runs in the background 24/7, collecting and extracting data from screen and audio input/output, can be frustrating. 

There are numerous use cases that can be built on top of this layer. To simplify life for other developers, we decided to solve this non-trivial problem. It's still in its early stages, but it works end-to-end. We're working on this full-time and would love to hear your feedback and suggestions.

## Get started

There are multiple ways to install screenpipe:
- as a CLI (continue reading), for rather technical users
- as a [paid desktop app](https://screenpi.pe) with 1 year updates, priority support, and priority features
- as a free forever desktop app (but you need to build it yourself). We're 100% OSS.
- as a free forever desktop app - by sending a PR ([example](https://github.com/mediar-ai/screenpipe/issues/120#issuecomment-2275043418)) (or offer free app to a friend)
- as a Rust or WASM library (documentation WIP)
- as a business - check [use cases](https://github.com/mediar-ai/screenpipe?tab=readme-ov-file#use-cases) and [DM louis](https://www.linkedin.com/in/louis030195/)

we invest 80% of the paid app revenue in [bounties](https://github.com/mediar-ai/screenpipe/issues?q=is:open+is:issue+label:%22%F0%9F%92%8E+Bounty%22), send PR, make money!

think of screenpipe like a DAO, but without crypto

This is the instructions to install the command line interface.

Struggle to get it running? [I'll install it with you in a 15 min call.](https://cal.com/louis030195/screenpipe)

<details>
  <summary>CLI installation</summary>


<details>
  <summary>MacOS</summary>

<details>
  <summary>Option I: brew</summary>

1. Install CLI
bash
brew tap mediar-ai/screenpipe https://github.com/mediar-ai/screenpipe.git
brew install screenpipe

2. Run it:
bash
screenpipe


if you don't want audio to be recorded
bash
screenpipe --disable-audio

if you want to save OCR data to text file in text_json folder in the root of your project (good for testing):
bash
screenpipe --save-text-files

if you want to run screenpipe in debug mode to show more logs in terminal:
bash
screenpipe --debug

by default screenpipe is using Silero-VAD to identify speech and non-speech tokens to improve audio transcription, bu you can use WebRTC if needed by passing the following command:
bash
screenpipe --vad-engine webrtc

by default screenpipe is using whisper-large that runs LOCALLY to get better quality or lower compute you can use cloud model (we use Deepgram) via cloud api:
bash
screenpipe --audio-transcription-engine deepgram


friend wearable integration, in order to link your wearable you need to pass user ID from friend app:
bash
screenpipe --friend-wearable-uid AC...........................F3


you can combine multiple flags if needed

[Didn't work?](https://github.com/mediar-ai/screenpipe/issues/new?assignees=&labels=dislike&template=dislike.yml&title=brew+install+screenpipe+didnt+work)

</details>

<details>
  <summary>Option II: Install from the source</summary>

1. Install dependencies:
bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # takes 5 minutes
brew install pkg-config ffmpeg jq tesseract


2. Clone the repo:

bash
git clone https://github.com/mediar-ai/screenpipe


This runs a local SQLite DB + an API + screenshot, ocr, mic, stt, mp4 encoding
bash
cd screenpipe # enter cloned repo


Build the project, takes 5-10 minutes depending on your hardware
bash
# necessary to use apple native OCR
export RUSTFLAGS="-C link-arg=-Wl,-rpath,@executable_path/../../screenpipe-vision/bin -C link-arg=-Wl,-rpath,@loader_path/../../screenpipe-vision/lib"
cargo build --release --features metal # takes 3 minuttes


Then run it
bash
./target/release/screenpipe # add --ocr-engine apple-native to use apple native OCR
# add "--disable-audio" if you don't want audio to be recorded
# "--save-text-files" if you want to save OCR data to text file in text_json folder in the root of your project (good for testing)
# "--debug" if you want to run screenpipe in debug mode to show more logs in terminal


[Didn't work?](https://github.com/mediar-ai/screenpipe/issues/new?assignees=&labels=dislike&template=dislike.yml&title=cloning+screenpipe+didnt+work)
</details>

<br><br>
</details>

<details>
  <summary>Windows</summary>


> [!note]
> This is experimental support for Windows build. This assumes you already have the CUDA Toolkit installed and the CUDA_PATH set to my CUDA v12.6 folder.
> Replace V:\projects and V:\packages with your own folders.

If this does not work for you, please [open an issue](https://github.com/mediar-ai/screenpipe/issues/new?assignees=&labels=dislike&template=dislike.yml&title=windows+install+screenpipe+didnt+work) or get the pre-built [desktop app](https://screenpi.pe)

- Install chocolatey
- Install git
- Install CUDA Toolkit (if using NVIDIA and building with cuda)
- Install MS Visual Studio Build Tools (below are the components I have installed)
	- Desktop development with C++
		- MSVC v143
		- Windows 11 SDK
		- C++ Cmake tools for Windows
		- Testing tools core features - Build tools
		- C++ AddressSanitizer
		- C++ ATL for latest v143
	- Individual components
		- C++ ATL for latest v143 build tools (x86 & x64)
		- MSBuild support for LLVM (clang-c) toolset
		- C++ Clang Compiler for Windows

batch
choco install pkgconfiglite rust
cd V:\projects
git clone https://github.com/mediar-ai/screenpipe
cd V:\packages
git clone https://github.com/microsoft/vcpkg.git
cd vcpkg
bootstrap-vcpkg.bat -disableMetrics
vcpkg.exe integrate install --disable-metrics
vcpkg.exe install ffmpeg

SET PKG_CONFIG_PATH=V:\packages\vcpkg\packages\ffmpeg_x64-windows\lib\pkgconfig
SET VCPKG_ROOT=V:\packages\vcpkg
SET LIBCLANG_PATH=C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\Llvm\x64\bin
cd V:\projects\screen-pipe

cargo build --release --features cuda

   
</details>

<details>
  <summary>Linux</summary>

<details>
  <summary>Option I: Install from source</summary>

1. Install dependencies:

bash
sudo apt-get update
sudo apt-get install -y libavformat-dev libavfilter-dev libavdevice-dev ffmpeg libasound2-dev tesseract-ocr libtesseract-dev

# Install Rust programming language
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


2. Clone the repo:

bash
git clone https://github.com/mediar-ai/screenpipe
cd screenpipe


3. Build and run:

bash
cargo build --release --features cuda # remove "--features cuda" if you do not have a NVIDIA GPU

# then run it
./target/release/screenpipe


</details>

<details>
  <summary>Option II: Install through Nix</summary>

Choose one of the following methods:

a. Using nix-env:
bash
nix-env -iA nixpkgs.screen-pipe


b. In your configuration.nix (for NixOS users):
Add the following to your configuration.nix:
nix
environment.systemPackages = with pkgs; [
  screen-pipe
];

Then rebuild your system with sudo nixos-rebuild switch.

c. In a Nix shell:
bash
nix-shell -p screen-pipe


d. Using nix run (for ad-hoc usage):
bash
nix run nixpkgs#screen-pipe


Note: Make sure you're using a recent version of nixpkgs that includes the screen-pipe package.

</details>
</details>

<br><br>
</details>

By default the data is stored in $HOME/.screenpipe (C:\AppData\Users\<user>\.screenpipe on Windows) you can change using --data-dir <mydir>

<details>
  <summary>run example vercel/ai chatbot web interface</summary>

  This example uses OpenAI. If you're looking for ollama example check the [examples folder](https://github.com/mediar-ai/screenpipe/tree/main/examples/typescript)

The [desktop app](https://screenpi.pe/) fully support OpenAI & Ollama by default.

To run Vercel chatbot, try this:

bash
git clone https://github.com/mediar-ai/screenpipe


Navigate to app directory
bash
cd screenpipe/examples/typescript/vercel-ai-chatbot

Set up you OPENAI API KEY in .env
bash
echo "OPENAI_API_KEY=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX" > .env

[Didn't work?](https://github.com/mediar-ai/screenpipe/issues/new?assignees=&labels=dislike&template=dislike.yml&title=vercel+app+didnt+work)

Install dependencies and run local web server
bash
npm install

bash
npm run dev

![Vercel App](./content/Vercel_app.png)
<br><br>

![Claude_prompt](./content/Claude_prompt.png)
<br><br>
</details>


## Usage

You can use terminal commands to query and view your data as shown below. Also, we recommend Tableplus.com to view the database, it has a free tier.

Here's a pseudo code to illustrate how to use screenpipe, to summarize meetings:
js
// 1h ago
const startDate = "<some time 1h ago..>"
// 10m ago
const endDate = "<some time 10m ago..>"

// get all the screen & mic data from roughly last hour 
const results = fetchScreenpipe(startDate, endDate)

// send it to an LLM and ask for a summary
const summary = fetchOllama("{results} create a summary from these transcriptions")
// or const summary = fetchOpenai(results)

// add the meeting summary to your notes
addToNotion(summary)
// or your favourite note taking app


Or thousands of other usages of all your screen & mic data! [Check examples](https://github.com/mediar-ai/screenpipe/tree/main/examples/typescript), screenpipes makes it easy to write plugins in JS for example that runs directly in the Rust code through Deno engine.


<details>
<summary>
Check which tables you have in the local database</summary>

bash
sqlite3 ~/.screenpipe/db.sqlite ".tables"

</details>
<details>
<summary>
Print a sample audio_transcriptions from the database</summary>

bash
sqlite3 ~/.screenpipe/db.sqlite ".mode json" ".once /dev/stdout" "SELECT * FROM audio_transcriptions ORDER BY id DESC LIMIT 1;" | jq .

![audio_transcriptions](./content/audio_transcriptions.png)
</details>
<details>
<summary>
Print a sample frame_OCR_text from the database</summary>

bash
sqlite3 ~/.screenpipe/db.sqlite ".mode json" ".once /dev/stdout" "SELECT * FROM ocr_text ORDER BY frame_id DESC LIMIT 1;" | jq -r '.[0].text'

![frame_text](./content/frame_text.png)
</details>
<details>
<summary>
Play a sample frame_recording from the database</summary>

bash
ffplay "data/2024-07-12_01-14-14.mp4"

</details>
<details>
<summary>
Play a sample audio_recording from the database</summary>

bash
ffplay "data/Display 1 (output)_2024-07-12_01-14-11.mp4"

</details>

<details>
  <summary>Example to query the API</summary>
  
1. Basic search query
bash
curl "http://localhost:3030/search?q=Neuralink&limit=5&offset=0&content_type=ocr" | jq

"Elon Musk" prompt
![Elon_Musk_prompt](./content/Elon_Musk_prompt.png)
</details>
<details>
  <summary>Other Example to query the API</summary>

  
bash
# 2. Search with content type filter (OCR)
curl "http://localhost:3030/search?q=QUERY_HERE&limit=5&offset=0&content_type=ocr"

# 3. Search with content type filter (Audio)
curl "http://localhost:3030/search?q=QUERY_HERE&limit=5&offset=0&content_type=audio"

# 4. Search with pagination
curl "http://localhost:3030/search?q=QUERY_HERE&limit=10&offset=20"

# 6. Search with no query (should return all results)
curl "http://localhost:3030/search?limit=5&offset=0"

# Display first frame from 30m to 25m ago (macos, translate to your OS with AI)
curl "http://localhost:3030/search?limit=1&offset=0&content_type=ocr&include_frames=true&start_time=$(date -u -v-220M +%Y-%m-%dT%H:%M:%SZ)&end_time=$(date -u -v-120M +%Y-%m-%dT%H:%M:%SZ)" | jq -r '.data[0].content.frame' | base64 --decode > /tmp/frame.png && open /tmp/frame.png

# filter by app (wll only return OCR results)
curl "http://localhost:3030/search?app_name=cursor"

</details>
<br><br>
Keep in mind that it's still experimental.
<br><br>

![0910244](https://github.com/user-attachments/assets/6025ef71-43b9-4151-a34c-155c30236a57)


## Use cases:

- Search
  - Semantic and keyword search. Find information you've forgotten or misplaced
  - Playback history of your desktop when searching for a specific info
- Automation: 
  - Automatically generate documentation
  - Populate CRM systems with relevant data
  - Synchronize company knowledge across platforms
  - Automate repetitive tasks based on screen content
- Analytics:
  - Track personal productivity metrics
  - Organize and analyze educational materials
  - Gain insights into areas for personal improvement
  - Analyze work patterns and optimize workflows
- Personal assistant:
  - Summarize lengthy documents or videos
  - Provide context-aware reminders and suggestions
  - Assist with research by aggregating relevant information
  - Live captions, translation support
- Collaboration:
  - Share and annotate screen captures with team members
  - Create searchable archives of meetings and presentations
- Compliance and security:
  - Track what your employees are really up to
  - Monitor and log system activities for audit purposes
  - Detect potential security threats based on screen content

[**Check other examples**](https://github.com/mediar-ai/screenpipe/tree/main/examples/typescript)

## Status 

Alpha: runs on my computer Macbook pro m3 32 GB ram and a $400 Windows laptop, 24/7.

Uses 600 MB, 10% CPU.

- [ ] Integrations
    - [x] ollama
    - [x] openai
    - [x] Friend wearable 
    - [x] [Fileorganizer2000](https://github.com/different-ai/file-organizer-2000)
    - [x] mem0
    - [x] Brilliant Frames
    - [x] Vercel AI SDK
    - [ ] supermemory
    - [x] deepgram
    - [x] unstructured
    - [x] excalidraw
    - [x] Obsidian
    - [x] Apple shortcut
    - [x] multion
    - [x] iPhone
    - [ ] Android
    - [ ] Camera
    - [ ] Keyboard
    - [x] Browser
    - [x] Pipe Store (a list of "pipes" you can build, share & easily install to get more value out of your screen & mic data without effort). It runs in Deno Typescript engine within screenpipe on your computer
- [x] screenshots + OCR with different engines to optimise privacy, quality, or energy consumption
  - [x] tesseract
  - [x] Windows native OCR
  - [x] Apple native OCR
  - [x] unstructured.io
  - [ ] screenpipe screen/audio specialised LLM
- [x] audio + STT (works with multi input devices, like your iPhone + mac mic, many STT engines)
  - [x] Linux, MacOS, Windows input & output devices
  - [x] iPhone microphone
- [x] [remote capture](https://github.com/mediar-ai/screenpipe/discussions/68) (run screenpipe on your cloud and it capture your local machine, only tested on Linux) for example when you have low compute laptop
- [x] optimised screen & audio recording (mp4 encoding, estimating 30 gb/m with default settings)
- [x] sqlite local db
- [x] local api
- [x] Cross platform CLI, [desktop app](https://screenpi.pe/) (MacOS, Windows, Linux)
- [x] Metal, CUDA
- [ ] TS SDK
- [ ] multimodal embeddings
- [ ] cloud storage options (s3, pgsql, etc.)
- [x] cloud computing options (deepgram for audio, unstructured for OCR)
- [x] custom storage settings: customizable capture settings (fps, resolution)
- [ ] security
  - [x] window specific capture (e.g. can decide to only capture specific tab of cursor, chrome, obsidian, or only specific app)
  - [ ] encryption
  - [x] PII removal
- [ ] fast, optimised, energy-efficient modes
- [ ] webhooks/events (for automations)
- [ ] abstractions for multiplayer usage (e.g. aggregate sales team data, company team data, partner, etc.)

## Why open source?

Recent breakthroughs in AI have shown that context is the final frontier. AI will soon be able to incorporate the context of an entire human life into its 'prompt', and the technologies that enable this kind of personalisation should be available to all developers to accelerate access to the next stage of our evolution.  

## Contributing

Contributions are welcome! If you'd like to contribute, please read [CONTRIBUTING.md](CONTRIBUTING.md).

## FAQ

<details>
  <summary>What's the difference with adept.ai and rewind.ai?</summary>

  - adept.ai is a closed product, focused on automation while we are open and focused on enabling tooling & infra for a wide range of applications like adept 
  - rewind.ai is a closed product, focused on a single use case (they only focus on meetings now), not customisable, your data is owned by them, and not extendable by developers 

</details>

<details>
  <summary>Where is the data stored?</summary>
  
  - 100% of the data stay local in a SQLite database and mp4/mp3 files. You own your data
</details>

<details>
  <summary>Do you encrypt the data?</summary>
  
  - Not yet but we're working on it. We want to provide you the highest level of security.
</details>

<details>
  <summary>How can I customize capture settings to reduce storage and energy usage?</summary>
  
  - You can adjust frame rates and resolution in the configuration. Lower values will reduce storage and energy consumption. We're working on making this more user-friendly in future updates.
</details>

<details>
  <summary>What are some practical use cases for screenpipe?</summary>
  
    - RAG & question answering
    - Automation (write code somewhere else while watching you coding, write docs, fill your CRM, sync company's knowledge, etc.)
    - Analytics (track human performance, education, become aware of how you can improve, etc.)
    - etc.
    - We're constantly exploring new use cases and welcome community input!
</details>

----

This is how we build the windows app in an automated way:


https://github.com/louis030195/screen-pipe/blob/main/.github/workflows/release-app.yml


We have a script that pre install some things that require bun to run 

screenpipe-app-tauri/scripts/pre_build.js:
import { $ } from 'bun'
import fs from 'fs/promises'
import os from 'os'
import path from 'path'

const isDevMode = process.env.SCREENPIPE_APP_DEV === 'true' || false;

const originalCWD = process.cwd()
// Change CWD to src-tauri
process.chdir(path.join(__dirname, '../src-tauri'))
const platform = {
	win32: 'windows',
	darwin: 'macos',
	linux: 'linux',
}[os.platform()]
const cwd = process.cwd()
console.log('cwd', cwd)
function hasFeature(name) {
	return process.argv.includes(`--${name}`) || process.argv.includes(name)
}

const config = {
	ffmpegRealname: 'ffmpeg',
	openblasRealname: 'openblas',
	clblastRealname: 'clblast',
	windows: { // TODO probably windows lack mp3
		ffmpegName: 'ffmpeg-7.0-windows-desktop-vs2022-default',
		ffmpegUrl: 'https://unlimited.dl.sourceforge.net/project/avbuild/windows-desktop/ffmpeg-7.0-windows-desktop-vs2022-default.7z?viasf=1',

		openBlasName: 'OpenBLAS-0.3.26-x64',
		openBlasUrl: 'https://github.com/OpenMathLib/OpenBLAS/releases/download/v0.3.26/OpenBLAS-0.3.26-x64.zip',

		clblastName: 'CLBlast-1.6.2-windows-x64',
		clblastUrl: 'https://github.com/CNugteren/CLBlast/releases/download/1.6.2/CLBlast-1.6.2-windows-x64.zip',

		vcpkgPackages: ['opencl', 'onnxruntime-gpu'],
	},
	linux: {
		aptPackages: [
			'tesseract-ocr',
			'libtesseract-dev',
			'ffmpeg',
			'libopenblas-dev', // Runtime
			'pkg-config',
			'build-essential',
			'libglib2.0-dev',
			'libgtk-3-dev',
			'libwebkit2gtk-4.1-dev',
			'clang',
			'cmake', // Tauri
			'libavutil-dev',
			'libavformat-dev',
			'libavfilter-dev',
			'libavdevice-dev', // FFMPEG
			'libasound2-dev', // cpal
			'libomp-dev', // OpenMP in ggml.ai
			'libstdc++-12-dev', //ROCm
		],
	},
	macos: {
		ffmpegName: 'ffmpeg-7.0-macOS-default',
		ffmpegUrl: 'https://master.dl.sourceforge.net/project/avbuild/macOS/ffmpeg-7.0-macOS-default.tar.xz?viasf=1',
	},
}

async function findWget() {
	const possiblePaths = [
		'C:\\ProgramData\\chocolatey\\bin\\wget.exe',
		'C:\\Program Files\\Git\\mingw64\\bin\\wget.exe',
		'C:\\msys64\\usr\\bin\\wget.exe',
		'C:\\Windows\\System32\\wget.exe',
		'wget' // This will work if wget is in PATH
	];

	for (const wgetPath of possiblePaths) {
		try {
			await $`${wgetPath} --version`.quiet();
			console.log(`wget found at: ${wgetPath}`);
			return wgetPath;
		} catch (error) {
			// wget not found at this path, continue searching
		}
	}

	console.error('wget not found. Please install wget and make sure it\'s in your PATH.');
	process.exit(1);
}

// Export for Github actions
const exports = {
	ffmpeg: path.join(cwd, config.ffmpegRealname),
	openBlas: path.join(cwd, config.openblasRealname),
	clblast: path.join(cwd, config.clblastRealname, 'lib/cmake/CLBlast'),
	libClang: 'C:\\Program Files\\LLVM\\bin',
	cmake: 'C:\\Program Files\\CMake\\bin',
}

/* ########## Linux ########## */
if (platform == 'linux') {
	// Install APT packages
	await $`sudo apt-get update`
	if (hasFeature('opencl')) {
		config.linux.aptPackages.push('libclblast-dev')
	}
	for (const name of config.linux.aptPackages) {
		await $`sudo apt-get install -y ${name}`
	}

	// Copy screenpipe binary
	console.log('Copying screenpipe binary for Linux...');
	const potentialPaths = [
		path.join(__dirname, '..', '..', '..', '..', 'target', 'release', 'screenpipe'),
		path.join(__dirname, '..', '..', '..', '..', 'target', 'x86_64-unknown-linux-gnu', 'release', 'screenpipe'),
		path.join(__dirname, '..', '..', '..', 'target', 'release', 'screenpipe'),
		path.join(__dirname, '..', '..', 'target', 'release', 'screenpipe'),
		'/home/runner/work/screenpipe/screenpipe/target/release/screenpipe',
	];

	let copied = false;
	for (const screenpipeSrc of potentialPaths) {
		const screenpipeDest = path.join(cwd, 'screenpipe-x86_64-unknown-linux-gnu');
		try {
			await fs.copyFile(screenpipeSrc, screenpipeDest);
			console.log(`screenpipe binary copied successfully from ${screenpipeSrc}`);
			copied = true;
			break;
		} catch (error) {
			console.warn(`failed to copy screenpipe binary from ${screenpipeSrc}:`, error);
		}
	}

	if (!copied) {
		console.error("failed to copy screenpipe binary from any potential path.");
		// uncomment the following line if you want the script to exit on failure
		// process.exit(1);
	}


}

/* ########## Windows ########## */
if (platform == 'windows') {
	const wgetPath = await findWget();

	console.log('Copying screenpipe binary...');

	const potentialPaths = [
		path.join(__dirname, '..', '..', 'target', 'release', 'screenpipe.exe'),
		path.join(__dirname, '..', '..', 'target', 'x86_64-pc-windows-msvc', 'release', 'screenpipe.exe'),
		path.join(__dirname, '..', 'target', 'release', 'screenpipe.exe'),
		path.join(__dirname, '..', '..', 'target', 'release', 'screenpipe.exe'),
		'D:\\a\\screenpipe\\screenpipe\\target\\release\\screenpipe.exe',
	];

	let copied = false;
	for (const screenpipeSrc of potentialPaths) {
		const screenpipeDest = path.join(cwd, 'screenpipe-x86_64-pc-windows-msvc.exe');
		try {
			await fs.copyFile(screenpipeSrc, screenpipeDest);
			console.log(`Screenpipe binary copied successfully from ${screenpipeSrc}`);
			copied = true;
			break;
		} catch (error) {
			console.warn(`Failed to copy screenpipe binary from ${screenpipeSrc}:`, error);
		}
	}

	if (!copied) {
		console.error("Failed to copy screenpipe binary from any potential path.");
		// Uncomment the following line if you want the script to exit on failure
		// process.exit(1);
	}


	// Setup FFMPEG
	if (!(await fs.exists(config.ffmpegRealname))) {
		await $`${wgetPath} -nc  --no-check-certificate --show-progress ${config.windows.ffmpegUrl} -O ${config.windows.ffmpegName}.7z`
		await $`'C:\\Program Files\\7-Zip\\7z.exe' x ${config.windows.ffmpegName}.7z`
		await $`mv ${config.windows.ffmpegName} ${config.ffmpegRealname}`
		await $`rm -rf ${config.windows.ffmpegName}.7z`
		await $`mv ${config.ffmpegRealname}/lib/x64/* ${config.ffmpegRealname}/lib/`
	}

	// Setup Tesseract
	const tesseractName = 'tesseract-setup'
	const tesseractUrl = 'https://github.com/UB-Mannheim/tesseract/releases/download/v5.4.0.20240606/tesseract-ocr-w64-setup-5.4.0.20240606.exe'
	const tesseractInstaller = `${tesseractName}.exe`

	if (!(await fs.exists('tesseract'))) {
		console.log('Setting up Tesseract for Windows...')
		await $`${wgetPath} -nc  --no-check-certificate --show-progress ${tesseractUrl} -O ${tesseractInstaller}`
		await $`"${process.cwd()}\\${tesseractInstaller}" /S /D=C:\\Program Files\\Tesseract-OCR`
		await $`rm ${tesseractInstaller}`
		// Replace the mv command with xcopy
		await $`xcopy "C:\\Program Files\\Tesseract-OCR" tesseract /E /I /H /Y`
		// Optionally, remove the original directory if needed
		// await $`rmdir "C:\\Program Files\\Tesseract-OCR" /S /Q`
		console.log('Tesseract for Windows set up successfully.')
	} else {
		console.log('Tesseract for Windows already exists.')
	}

	// Add Tesseract to PATH
	process.env.PATH = `${process.cwd()}\\tesseract;${process.env.PATH}`

	// Setup ONNX Runtime
	const onnxRuntimeName = "onnxruntime-win-x64-gpu-1.19.2";
	const onnxRuntimeLibs = `${onnxRuntimeName}.zip`;
	const onnxRuntimeUrl = `https://github.com/microsoft/onnxruntime/releases/download/v1.19.2/${onnxRuntimeLibs}`
	if (!(await fs.exists(onnxRuntimeName))) {
		console.log('Setting up ONNX Runtime libraries for Windows...')
		await $`${wgetPath} -nc  --no-check-certificate --show-progress ${onnxRuntimeUrl} -O ${onnxRuntimeLibs}`
		await $`unzip ${onnxRuntimeLibs} || tar -xf ${onnxRuntimeLibs} || echo "Done extracting"`;
		await $`rm -rf ${onnxRuntimeLibs} || rm ${onnxRuntimeLibs} -Recurse -Force || echo "Done cleaning up zip"`;
		console.log('ONNX Runtime libraries for Windows set up successfully.')
	} else {
		console.log('ONNX Runtime libraries for Windows already exists.')
	}

	// Setup OpenBlas
	if (!(await fs.exists(config.openblasRealname)) && hasFeature('openblas')) {
		await $`${wgetPath} -nc --show-progress ${config.windows.openBlasUrl} -O ${config.windows.openBlasName}.zip`
		await $`"C:\\Program Files\\7-Zip\\7z.exe" x ${config.windows.openBlasName}.zip -o${config.openblasRealname}`
		await $`rm ${config.windows.openBlasName}.zip`
		fs.cp(path.join(config.openblasRealname, 'include'), path.join(config.openblasRealname, 'lib'), { recursive: true, force: true })
		// It tries to link only openblas.lib but our is libopenblas.lib`
		fs.cp(path.join(config.openblasRealname, 'lib/libopenblas.lib'), path.join(config.openblasRealname, 'lib/openblas.lib'))
	}

	// Setup CLBlast
	if (!(await fs.exists(config.clblastRealname)) && !hasFeature('cuda')) {
		await $`${wgetPath} -nc --show-progress ${config.windows.clblastUrl} -O ${config.windows.clblastName}.zip`
		await $`"C:\\Program Files\\7-Zip\\7z.exe" x ${config.windows.clblastName}.zip` // 7z file inside
		await $`"C:\\Program Files\\7-Zip\\7z.exe" x ${config.windows.clblastName}.7z` // Inner folder
		await $`mv ${config.windows.clblastName} ${config.clblastRealname}`
		await $`rm ${config.windows.clblastName}.zip`
		await $`rm ${config.windows.clblastName}.7z`
	}

	// Setup vcpkg packages
	await $`C:\\vcpkg\\vcpkg.exe install ${config.windows.vcpkgPackages}`.quiet()
}

async function getMostRecentBinaryPath(targetArch, paths) {
	const validPaths = await Promise.all(paths.map(async (path) => {
		if (await fs.exists(path)) {
			const { stdout } = await $`file ${path}`.quiet();
			const binaryArch = stdout.includes('arm64') ? 'arm64' :
				stdout.includes('x86_64') ? 'x86_64' : null;
			if (binaryArch === targetArch) {
				const stat = await fs.stat(path);
				return { path, mtime: stat.mtime };
			}
		}
		return null;
	}));

	const filteredPaths = validPaths.filter(Boolean);

	if (filteredPaths.length === 0) {
		return null;
	}

	return filteredPaths.reduce((mostRecent, current) =>
		current.mtime > mostRecent.mtime ? current : mostRecent
	).path;
}
/* ########## macOS ########## */
if (platform == 'macos') {

	const architectures = ['arm64', 'x86_64'];

	for (const arch of architectures) {
		if (process.env['SKIP_SCREENPIPE_SETUP']) {
			break;
		}
		console.log(`Setting up screenpipe bin for ${arch}...`);

		if (arch === 'arm64') {
			const paths = [
				"../../target/aarch64-apple-darwin/release/screenpipe",
				"../../target/release/screenpipe"
			];

			const mostRecentPath = await getMostRecentBinaryPath('arm64', paths);
			if (mostRecentPath) {
				await $`cp ${mostRecentPath} screenpipe-aarch64-apple-darwin`;
				console.log(`Copied most recent arm64 screenpipe binary from ${mostRecentPath}`);
			} else {
				console.error("No suitable arm64 screenpipe binary found");
			}
			// if the binary exists, hard code the fucking dylib
			if (await fs.exists('screenpipe-aarch64-apple-darwin') && !isDevMode) {
				await $`install_name_tool -change screenpipe-vision/lib/libscreenpipe_arm64.dylib @rpath/../Frameworks/libscreenpipe_arm64.dylib ./screenpipe-aarch64-apple-darwin`
				await $`install_name_tool -change screenpipe-vision/lib/libscreenpipe.dylib @rpath/../Frameworks/libscreenpipe.dylib ./screenpipe-aarch64-apple-darwin`
				console.log(`hard coded the FUCKING dylib`);
			} else if (await fs.exists('screenpipe-aarch64-apple-darwin') && isDevMode) {
				await $`install_name_tool -change screenpipe-vision/lib/libscreenpipe_arm64.dylib @executable_path/../Frameworks/libscreenpipe_arm64.dylib ./screenpipe-aarch64-apple-darwin`
				await $`install_name_tool -change screenpipe-vision/lib/libscreenpipe.dylib @executable_path/../Frameworks/libscreenpipe.dylib ./screenpipe-aarch64-apple-darwin`
				await $`install_name_tool -add_rpath @executable_path/../Frameworks ./screenpipe-aarch64-apple-darwin`
				console.log(`Updated dylib paths for arm64 in dev mode`);
			}


		} else if (arch === 'x86_64') {
			// copy screenpipe binary (more recent one)
			const paths = [
				"../../target/x86_64-apple-darwin/release/screenpipe",
				"../../target/release/screenpipe"
			];

			const mostRecentPath = await getMostRecentBinaryPath('x86_64', paths);

			if (mostRecentPath) {
				await $`cp ${mostRecentPath} screenpipe-x86_64-apple-darwin`;
				console.log(`Copied most recent x86_64 screenpipe binary from ${mostRecentPath}`);
			} else {
				console.error("No suitable x86_64 screenpipe binary found");
			}
			// hard code the fucking dylib
			if (await fs.exists('screenpipe-x86_64-apple-darwin') && !isDevMode) {
				await $`install_name_tool -change screenpipe-vision/lib/libscreenpipe_x86_64.dylib @rpath/../Frameworks/libscreenpipe_x86_64.dylib ./screenpipe-x86_64-apple-darwin`
				await $`install_name_tool -change screenpipe-vision/lib/libscreenpipe.dylib @rpath/../Frameworks/libscreenpipe.dylib ./screenpipe-x86_64-apple-darwin`
				console.log(`hard coded the FUCKING dylib`);
			}

		}

		console.log(`screenpipe for ${arch} set up successfully.`);
	}


	// Setup FFMPEG
	if (!(await fs.exists(config.ffmpegRealname))) {
		await $`wget -nc ${config.macos.ffmpegUrl} -O ${config.macos.ffmpegName}.tar.xz`
		await $`tar xf ${config.macos.ffmpegName}.tar.xz`
		await $`mv ${config.macos.ffmpegName} ${config.ffmpegRealname}`
		await $`rm ${config.macos.ffmpegName}.tar.xz`
	} else {
		console.log('FFMPEG already exists');
	}

	// Move and rename ffmpeg and ffprobe binaries
	const ffmpegSrc = path.join(cwd, config.ffmpegRealname, 'bin', 'ffmpeg');

	// For x86_64
	await fs.copyFile(ffmpegSrc, path.join(cwd, 'ffmpeg-x86_64-apple-darwin'));

	// For arm64
	await fs.copyFile(ffmpegSrc, path.join(cwd, 'ffmpeg-aarch64-apple-darwin'));

	console.log('Moved and renamed ffmpeg binary for externalBin');
}

// Nvidia
let cudaPath
if (hasFeature('cuda')) {
	if (process.env['CUDA_PATH']) {
		cudaPath = process.env['CUDA_PATH']
	} else if (platform === 'windows') {
		const cudaRoot = 'C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\'
		cudaPath = 'C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v12.5'
		if (await fs.exists(cudaRoot)) {
			const folders = await fs.readdir(cudaRoot)
			if (folders.length > 0) {
				cudaPath = cudaPath.replace('v12.5', folders[0])
			}
		}
	}

	if (process.env.GITHUB_ENV) {
		console.log('CUDA_PATH', cudaPath)
	}

	if (platform === 'windows') {
		const windowsConfig = {
			bundle: {
				resources: {
					'ffmpeg\\bin\\x64\\*': './',
					'openblas\\bin\\*.dll': './',
					[`${cudaPath}\\bin\\cudart64_*`]: './',
					[`${cudaPath}\\bin\\cublas64_*`]: './',
					[`${cudaPath}\\bin\\cublasLt64_*`]: './',
					'tesseract\\*': './',
					'onnxruntime*\\lib\\*.dll': './',
				},
				externalBin: [
					'screenpipe'
				]
			},
		}
		await fs.writeFile('tauri.windows.conf.json', JSON.stringify(windowsConfig, null, 4))
	}
	if (platform === 'linux') {
		// Add cuda toolkit depends package
		const tauriConfigContent = await fs.readFile('tauri.linux.conf.json', { encoding: 'utf-8' })
		const tauriConfig = JSON.parse(tauriConfigContent)
		tauriConfig.bundle.linux.deb.depends.push('nvidia-cuda-toolkit')
		await fs.writeFile('tauri.linux.conf.json', JSON.stringify(tauriConfig, null, 4))
	}
}

if (hasFeature('opencl')) {
	if (platform === 'windows') {
		const tauriConfigContent = await fs.readFile('tauri.windows.conf.json', { encoding: 'utf-8' })
		const tauriConfig = JSON.parse(tauriConfigContent)
		tauriConfig.bundle.resources['clblast\\bin\\*.dll'] = './'
		tauriConfig.bundle.resources['C:\\vcpkg\\packages\\opencl_x64-windows\\bin\\*.dll'] = './'
		await fs.writeFile('tauri.windows.conf.json', JSON.stringify(tauriConfig, null, 4))
	}
}

// OpenBlas
if (hasFeature('openblas')) {
	if (platform === 'windows') {
		const tauriConfigContent = await fs.readFile('tauri.windows.conf.json', { encoding: 'utf-8' })
		const tauriConfig = JSON.parse(tauriConfigContent)
		tauriConfig.bundle.resources['openblas\\bin\\*.dll'] = './'
		await fs.writeFile('tauri.windows.conf.json', JSON.stringify(tauriConfig, null, 4))
	}
}

// ROCM
let rocmPath = '/opt/rocm'
if (hasFeature('rocm')) {
	if (process.env.GITHUB_ENV) {
		console.log('ROCM_PATH', rocmPath)
	}
	if (platform === 'linux') {
		// Add rocm toolkit depends package
		const tauriConfigContent = await fs.readFile('tauri.linux.conf.json', { encoding: 'utf-8' })
		const tauriConfig = JSON.parse(tauriConfigContent)
		tauriConfig.bundle.linux.deb.depends.push('rocm')
		await fs.writeFile('tauri.linux.conf.json', JSON.stringify(tauriConfig, null, 4))
	}
}

// Development hints
if (!process.env.GITHUB_ENV) {
	console.log('\nCommands to build 🔨:')
	// Get relative path to screenpipe-app-tauri folder
	const relativePath = path.relative(originalCWD, path.join(cwd, '..'))
	if (originalCWD != cwd && relativePath != '') {
		console.log(`cd ${relativePath}`)
	}
	console.log('bun install')
	if (platform == 'windows') {
		console.log(`$env:FFMPEG_DIR = "${exports.ffmpeg}"`)
		console.log(`$env:OPENBLAS_PATH = "${exports.openBlas}"`)
		console.log(`$env:LIBCLANG_PATH = "${exports.libClang}"`)
		console.log(`$env:PATH += "${exports.cmake}"`)
		if (hasFeature('older-cpu')) {
			console.log(`$env:WHISPER_NO_AVX = "ON"`)
			console.log(`$env:WHISPER_NO_AVX2 = "ON"`)
			console.log(`$env:WHISPER_NO_FMA = "ON"`)
			console.log(`$env:WHISPER_NO_F16C = "ON"`)
		}
		if (hasFeature('cuda')) {
			console.log(`$env:CUDA_PATH = "${cudaPath}"`)
		}
		if (hasFeature('opencl')) {
			console.log(`$env:CLBlast_DIR = "${exports.clblast}"`)
		}
		if (hasFeature('rocm')) {
			console.log(`$env:ROCM_VERSION = "6.1.2"`)
			console.log(`$env:ROCM_PATH = "${rocmPath}"`)
		}
	}
	if (!process.env.GITHUB_ENV) {
		console.log('bun tauri build')
	}
}

// Config Github ENV
if (process.env.GITHUB_ENV) {
	console.log('Adding ENV')
	if (platform == 'macos' || platform == 'windows') {
		const ffmpeg = `FFMPEG_DIR=${exports.ffmpeg}\n`
		console.log('Adding ENV', ffmpeg)
		await fs.appendFile(process.env.GITHUB_ENV, ffmpeg)
	}
	if (platform == 'macos') {
		const embed_metal = 'WHISPER_METAL_EMBED_LIBRARY=ON'
		await fs.appendFile(process.env.GITHUB_ENV, embed_metal)
	}
	if (platform == 'windows') {
		const openblas = `OPENBLAS_PATH=${exports.openBlas}\n`
		console.log('Adding ENV', openblas)
		await fs.appendFile(process.env.GITHUB_ENV, openblas)

		if (hasFeature('opencl')) {
			const clblast = `CLBlast_DIR=${exports.clblast}\n`
			console.log('Adding ENV', clblast)
			await fs.appendFile(process.env.GITHUB_ENV, clblast)
		}

		if (hasFeature('older-cpu')) {
			await fs.appendFile(process.env.GITHUB_ENV, `WHISPER_NO_AVX=ON\n`)
			await fs.appendFile(process.env.GITHUB_ENV, `WHISPER_NO_AVX2=ON\n`)
			await fs.appendFile(process.env.GITHUB_ENV, `WHISPER_NO_FMA=ON\n`)
			await fs.appendFile(process.env.GITHUB_ENV, `WHISPER_NO_F16C=ON\n`)
		}
	}
}

// --dev or --build
const action = process.argv?.[2]
if (action?.includes('--build' || action.includes('--dev'))) {
	process.chdir(path.join(cwd, '..'))
	process.env['FFMPEG_DIR'] = exports.ffmpeg
	if (platform === 'windows') {
		process.env['OPENBLAS_PATH'] = exports.openBlas
		process.env['CLBlast_DIR'] = exports.clblast
		process.env['LIBCLANG_PATH'] = exports.libClang
		process.env['PATH'] = `${process.env['PATH']};${exports.cmake}`
	}
	if (platform == 'macos') {
		process.env['WHISPER_METAL_EMBED_LIBRARY'] = 'ON'
	}
	await $`bun install`
	await $`bunx tauri ${action.includes('--dev') ? 'dev' : 'build'}`
}


---------------
Hey Josh, happy to help you with building the app from sources for arm


Feel free to join the Discord and DMing me at louis030195 (would be more efficient there)


To build from source we have some instructions on the readme but i'm afraid a few things are missing (trying to update the docs)


You probably need visual studio c++, rust, ffmpeg, git, pkgconfig, and the tricky part is to properly set environment variables to make the build work


You might need to install tesseract (an OCR engine) but ideally you use Windows native OCR which is better but we still have some reported bugs with it


This is how we build the windows app in an automated way:


https://github.com/louis030195/screen-pipe/blob/main/.github/workflows/release-app.yml



We have a script that pre install some things that require bun to run 


So in theory you can just install visual studio c++, rust, git, bun, and run "bun scripts/pre_build.js" from the "examples/apps/screenpipe-app-tauri" directory
and then "bun tauri build" and it would work. However in practice I expect that you run into some issues like missing environment variables


Several windows users managed to build from source (also myself in a VM), asking another windows user who built RN if he has clearer instructions



-------


I am trying to install screenpipe for my snapdragon x elite arm64 processor.

Please review these suggestions and then provide a comprehensive step by step guide for completing the build for my pc.
ChatGPT

Thought for 32 seconds

Comprehensive Guide to Building Screenpipe on Windows ARM64 (Snapdragon X Elite)
Note: Building Screenpipe on Windows ARM64 is experimental and may involve additional challenges due to limited support for ARM64 in some tools. This guide aims to help you through the process, but please be aware that some steps may require troubleshooting.

Table of Contents
Prerequisites
Installing Dependencies
1. Visual Studio 2022 Build Tools
2. Rust Programming Language
3. Git for Windows
4. pkg-config
5. FFmpeg
6. Tesseract OCR
7. bun (JavaScript Runtime)
Cloning the Screenpipe Repository
Setting Up Environment Variables
Building Screenpipe
1. Building the Rust Project
2. Building the Tauri App
Troubleshooting
Alternative Approaches
Contact and Support
Summary
Prerequisites
Windows 10/11 ARM64 Edition
Internet Connection
Administrator Access
Installing Dependencies
1. Visual Studio 2022 Build Tools
Purpose: Provides C++ compilers and build tools required for building Rust projects and native dependencies.

Steps:

Download Visual Studio Build Tools:

Visit the Visual Studio Downloads page.
Under "All Downloads," select "Build Tools for Visual Studio 2022."
Install Required Components:

Run the installer (vs_BuildTools.exe).
In the Workloads tab, select:
Desktop development with C++
In the Installation details pane, ensure the following components are selected:
MSVC v143 - VS 2022 C++ ARM64 Build Tools (Latest)
Windows 11 SDK (10.0.22621.0) for ARM64
C++ CMake tools for Windows
Testing tools core features - Build Tools
C++ AddressSanitizer
C++ ATL for latest v143 ARM64
C++ Clang Compiler for Windows (ARM64)
Click Install to begin the installation.
2. Rust Programming Language
Purpose: Rust is the primary language used in Screenpipe.

Steps:

Download Rustup:

Visit rustup.rs.
Click on "Other Installation Methods" to find the installer for Windows ARM64.
Alternatively, in a PowerShell terminal, run:
powershell
Copy code
Invoke-WebRequest -Uri https://static.rust-lang.org/rustup/dist/aarch64-pc-windows-msvc/rustup-init.exe -OutFile rustup-init.exe
Install Rust:

Run rustup-init.exe.
Follow the prompts, selecting the default installation.
Ensure that the toolchain is set for stable-aarch64-pc-windows-msvc.
Verify Installation:

powershell
Copy code
rustc --version
3. Git for Windows
Purpose: Git is required to clone the Screenpipe repository.

Steps:

Download Git:

Visit Git for Windows.
Download the latest installer.
Install Git:

Run the installer.
Follow the installation wizard, accepting defaults.
Note: Git for Windows runs under emulation on ARM64 devices. This should not affect functionality.
Verify Installation:

powershell
Copy code
git --version
4. pkg-config
Purpose: pkg-config helps in managing library compile and link flags.

Steps:

Install Chocolatey (If Not Installed):

Open an elevated PowerShell (Run as Administrator).
Run:
powershell
Copy code
Set-ExecutionPolicy Bypass -Scope Process -Force; `
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; `
iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))
Install pkg-config:

powershell
Copy code
choco install pkgconfiglite -y
Add pkg-config to PATH:

powershell
Copy code
$env:PATH += ";C:\Program Files (x86)\pkgconfiglite\bin"
5. FFmpeg
Purpose: FFmpeg is used for handling multimedia data.

Steps:

Download FFmpeg ARM64 Build:

Visit Gyan.dev FFmpeg Builds.
Under Release Builds, check for ARM64 builds.
Note: If an official ARM64 build is not available, you may need to build FFmpeg from source.
Alternative: Build FFmpeg from Source:

Building FFmpeg on Windows ARM64 is complex and may require additional tools like MSYS2 or Cygwin.
Consider using a pre-built binary if possible.
Extract FFmpeg:

Extract the contents to C:\ffmpeg.
Verify FFmpeg:

powershell
Copy code
C:\ffmpeg\bin\ffmpeg.exe -version
6. Tesseract OCR
Purpose: Tesseract is an OCR engine used for extracting text from images.

Steps:

Download Tesseract:

Visit UB Mannheim Tesseract OCR.
Download the latest Windows installer.
Install Tesseract:

Run the installer.
Install to the default location (C:\Program Files\Tesseract-OCR).
Add Tesseract to PATH:

powershell
Copy code
$env:PATH += ";C:\Program Files\Tesseract-OCR"
Verify Installation:

powershell
Copy code
tesseract --version
























#########################################
#########################################

CONTINUE FROM HERE

#########################################
#########################################

7. bun (JavaScript Runtime)
Purpose: bun is used to run JavaScript scripts, particularly pre_build.js.

Steps:

Check bun Support for Windows ARM64:

As of now, bun does not officially support Windows ARM64.
Alternative: Use Node.js instead.
Install Node.js for Windows ARM64:

Visit Node.js Downloads.
Download the Windows Binary (.zip) for ARM64.
Extract the contents to C:\nodejs.
Add Node.js to PATH:
powershell
Copy code
$env:PATH += ";C:\nodejs"
Verify Installation:
powershell
Copy code
node --version
Modify Scripts to Use Node.js:

The pre_build.js script may need adjustments to run with Node.js instead of bun.
Cloning the Screenpipe Repository
Open PowerShell or Command Prompt.

Navigate to a Directory:

powershell
Copy code
cd C:\projects
Clone the Repository:

powershell
Copy code
git clone https://github.com/mediar-ai/screenpipe.git
Navigate to the Screenpipe Directory:

powershell
Copy code
cd screenpipe
Setting Up Environment Variables
FFmpeg:

powershell
Copy code
$env:FFMPEG_DIR = "C:\ffmpeg"
Tesseract:

powershell
Copy code
$env:TESSERACT_DIR = "C:\Program Files\Tesseract-OCR"
pkg-config:

powershell
Copy code
$env:PKG_CONFIG_PATH = "C:\ffmpeg\lib\pkgconfig"
Note: You may need to generate .pc files for FFmpeg if they are not provided.
Add to PATH:

powershell
Copy code
$env:PATH += ";C:\ffmpeg\bin;C:\Program Files\Tesseract-OCR;C:\Program Files (x86)\pkgconfiglite\bin"
Building Screenpipe
1. Building the Rust Project
Steps:

Ensure Rust is Installed and Up-to-Date:

powershell
Copy code
rustup update
Install Required Rust Components:

powershell
Copy code
rustup component add rustfmt clippy
Navigate to the Screenpipe Directory (if not already there):

powershell
Copy code
cd C:\projects\screenpipe
Attempt to Build:

powershell
Copy code
cargo build --release
Troubleshoot Build Errors:

Missing Dependencies:

If you receive errors about missing libavformat, ensure that FFmpeg's libraries are accessible.
You may need to adjust PKG_CONFIG_PATH or copy .lib files to appropriate locations.
pkg-config Errors:

Ensure that pkg-config is properly installed and in your PATH.
Generate or obtain .pc files for FFmpeg and place them in the directory specified by PKG_CONFIG_PATH.
Example of Generating FFmpeg .pc Files:

Create a file named libavformat.pc in C:\ffmpeg\lib\pkgconfig with the following content:
makefile
Copy code
prefix=C:/ffmpeg
exec_prefix=${prefix}
libdir=${exec_prefix}/lib
includedir=${prefix}/include

Name: libavformat
Description: FFmpeg libavformat library
Version: 4.4
Requires:
Libs: -L${libdir} -lavformat -lavcodec -lavutil
Cflags: -I${includedir}
2. Building the Tauri App
Steps:

Navigate to the Tauri App Directory:

powershell
Copy code
cd examples\apps\screenpipe-app-tauri
Install Dependencies:

powershell
Copy code
npm install
Modify pre_build.js for Node.js:

Open scripts\pre_build.js in a text editor.
Replace any bun specific code with Node.js equivalents.
Ensure that all required modules are imported correctly (e.g., use require instead of import if necessary).
Run the Pre-Build Script:

powershell
Copy code
node scripts\pre_build.js
Build the Tauri App:

powershell
Copy code
npx tauri build
Troubleshoot Build Errors:

Missing Binaries:

Ensure that the screenpipe executable built earlier is accessible.
Copy the built screenpipe.exe to the Tauri app's source directory if necessary.
Environment Variables:

Ensure that all environment variables are correctly set in the PowerShell session.
Troubleshooting
Dependency Issues:

Double-check that all dependencies are installed and correctly configured.
Ensure that environment variables point to the correct directories.
Architecture Mismatch:

Ensure that all tools and libraries are built for ARM64 architecture.
Mixing x86_64 and ARM64 binaries will lead to build failures.
pkg-config Not Finding Libraries:

Verify that .pc files exist for all required libraries.
Ensure PKG_CONFIG_PATH includes the directories containing .pc files.
FFmpeg Compilation:

If pre-built FFmpeg binaries are not available for ARM64, consider building from source.
Building FFmpeg from source on Windows ARM64 requires MSYS2 or a similar environment.
bun Not Available:

Use Node.js as an alternative and adjust scripts accordingly.
Alternative Approaches
1. Use Windows Subsystem for Linux (WSL)
Install WSL with Ubuntu ARM64:
powershell
Copy code
wsl --install -d Ubuntu
Follow Linux Build Instructions:
Install dependencies using apt.
Build Screenpipe within WSL.
Limitations:
GUI and hardware access (e.g., screen capture) may not work as expected in WSL.
2. Cross-Compilation
Cross-Compile from Another Platform:
Set up a cross-compilation environment on Linux.
Target Windows ARM64.
Complexity:
This approach is advanced and may not be practical for most users.
3. Use a Different Machine
Use a Windows x86_64 Machine:
If possible, build Screenpipe on a standard Windows PC.
Contact and Support
Discord: Join the Screenpipe Discord for real-time support.
GitHub Issues: Open an issue on the Screenpipe GitHub Repository detailing your challenges.
Email: Reach out via email to the maintainers for assistance.
Summary
Building Screenpipe on Windows ARM64 involves several steps and potential challenges due to limited support for ARM64 in some tools. This guide covers:

Installing necessary tools and dependencies.
Setting up environment variables.
Adjusting build scripts to accommodate differences in your environment.
Troubleshooting common issues.
Key Points:

Ensure All Tools are ARM64 Compatible: Mixing architectures will cause build failures.
Environment Variables are Crucial: They help the build system locate dependencies.
Building Dependencies from Source May Be Necessary: Especially for tools not readily available for ARM64.
Adjust Scripts as Needed: Some scripts may assume availability of certain tools (e.g., bun).
We hope this guide assists you in building Screenpipe on your Snapdragon X Elite ARM64 PC. If you encounter issues not covered here, please don't hesitate to reach out for support.






You said:
import { $ } from 'bun'
import fs from 'fs/promises'
import os from 'os'
import path from 'path'

const isDevMode = process.env.SCREENPIPE_APP_DEV === 'true' || false;

const originalCWD = process.cwd()
// Change CWD to src-tauri
process.chdir(path.join(__dirname, '../src-tauri'))
const platform = {
	win32: 'windows',
	darwin: 'macos',
	linux: 'linux',
}[os.platform()]
const cwd = process.cwd()
console.log('cwd', cwd)
function hasFeature(name) {
	return process.argv.includes(--${name}) || process.argv.includes(name)
}

const config = {
	ffmpegRealname: 'ffmpeg',
	openblasRealname: 'openblas',
	clblastRealname: 'clblast',
	windows: { // TODO probably windows lack mp3
		ffmpegName: 'ffmpeg-7.0-windows-desktop-vs2022-default',
		ffmpegUrl: 'https://unlimited.dl.sourceforge.net/project/avbuild/windows-desktop/ffmpeg-7.0-windows-desktop-vs2022-default.7z?viasf=1',

		openBlasName: 'OpenBLAS-0.3.26-x64',
		openBlasUrl: 'https://github.com/OpenMathLib/OpenBLAS/releases/download/v0.3.26/OpenBLAS-0.3.26-x64.zip',

		clblastName: 'CLBlast-1.6.2-windows-x64',
		clblastUrl: 'https://github.com/CNugteren/CLBlast/releases/download/1.6.2/CLBlast-1.6.2-windows-x64.zip',

		vcpkgPackages: ['opencl', 'onnxruntime-gpu'],
	},
	linux: {
		aptPackages: [
			'tesseract-ocr',
			'libtesseract-dev',
			'ffmpeg',
			'libopenblas-dev', // Runtime
			'pkg-config',
			'build-essential',
			'libglib2.0-dev',
			'libgtk-3-dev',
			'libwebkit2gtk-4.1-dev',
			'clang',
			'cmake', // Tauri
			'libavutil-dev',
			'libavformat-dev',
			'libavfilter-dev',
			'libavdevice-dev', // FFMPEG
			'libasound2-dev', // cpal
			'libomp-dev', // OpenMP in ggml.ai
			'libstdc++-12-dev', //ROCm
		],
	},
	macos: {
		ffmpegName: 'ffmpeg-7.0-macOS-default',
		ffmpegUrl: 'https://master.dl.sourceforge.net/project/avbuild/macOS/ffmpeg-7.0-macOS-default.tar.xz?viasf=1',
	},
}

async function findWget() {
	const possiblePaths = [
		'C:\\ProgramData\\chocolatey\\bin\\wget.exe',
		'C:\\Program Files\\Git\\mingw64\\bin\\wget.exe',
		'C:\\msys64\\usr\\bin\\wget.exe',
		'C:\\Windows\\System32\\wget.exe',
		'wget' // This will work if wget is in PATH
	];

	for (const wgetPath of possiblePaths) {
		try {
			await $${wgetPath} --version.quiet();
			console.log(wget found at: ${wgetPath});
			return wgetPath;
		} catch (error) {
			// wget not found at this path, continue searching
		}
	}

	console.error('wget not found. Please install wget and make sure it\'s in your PATH.');
	process.exit(1);
}

// Export for Github actions
const exports = {
	ffmpeg: path.join(cwd, config.ffmpegRealname),
	openBlas: path.join(cwd, config.openblasRealname),
	clblast: path.join(cwd, config.clblastRealname, 'lib/cmake/CLBlast'),
	libClang: 'C:\\Program Files\\LLVM\\bin',
	cmake: 'C:\\Program Files\\CMake\\bin',
}

/* ########## Linux ########## */
if (platform == 'linux') {
	// Install APT packages
	await $sudo apt-get update
	if (hasFeature('opencl')) {
		config.linux.aptPackages.push('libclblast-dev')
	}
	for (const name of config.linux.aptPackages) {
		await $sudo apt-get install -y ${name}
	}

	// Copy screenpipe binary
	console.log('Copying screenpipe binary for Linux...');
	const potentialPaths = [
		path.join(__dirname, '..', '..', '..', '..', 'target', 'release', 'screenpipe'),
		path.join(__dirname, '..', '..', '..', '..', 'target', 'x86_64-unknown-linux-gnu', 'release', 'screenpipe'),
		path.join(__dirname, '..', '..', '..', 'target', 'release', 'screenpipe'),
		path.join(__dirname, '..', '..', 'target', 'release', 'screenpipe'),
		'/home/runner/work/screenpipe/screenpipe/target/release/screenpipe',
	];

	let copied = false;
	for (const screenpipeSrc of potentialPaths) {
		const screenpipeDest = path.join(cwd, 'screenpipe-x86_64-unknown-linux-gnu');
		try {
			await fs.copyFile(screenpipeSrc, screenpipeDest);
			console.log(screenpipe binary copied successfully from ${screenpipeSrc});
			copied = true;
			break;
		} catch (error) {
			console.warn(failed to copy screenpipe binary from ${screenpipeSrc}:, error);
		}
	}

	if (!copied) {
		console.error("failed to copy screenpipe binary from any potential path.");
		// uncomment the following line if you want the script to exit on failure
		// process.exit(1);
	}


}

/* ########## Windows ########## */
if (platform == 'windows') {
	const wgetPath = await findWget();

	console.log('Copying screenpipe binary...');

	const potentialPaths = [
		path.join(__dirname, '..', '..', 'target', 'release', 'screenpipe.exe'),
		path.join(__dirname, '..', '..', 'target', 'x86_64-pc-windows-msvc', 'release', 'screenpipe.exe'),
		path.join(__dirname, '..', 'target', 'release', 'screenpipe.exe'),
		path.join(__dirname, '..', '..', 'target', 'release', 'screenpipe.exe'),
		'D:\\a\\screenpipe\\screenpipe\\target\\release\\screenpipe.exe',
	];

	let copied = false;
	for (const screenpipeSrc of potentialPaths) {
		const screenpipeDest = path.join(cwd, 'screenpipe-x86_64-pc-windows-msvc.exe');
		try {
			await fs.copyFile(screenpipeSrc, screenpipeDest);
			console.log(Screenpipe binary copied successfully from ${screenpipeSrc});
			copied = true;
			break;
		} catch (error) {
			console.warn(Failed to copy screenpipe binary from ${screenpipeSrc}:, error);
		}
	}

	if (!copied) {
		console.error("Failed to copy screenpipe binary from any potential path.");
		// Uncomment the following line if you want the script to exit on failure
		// process.exit(1);
	}


	// Setup FFMPEG
	if (!(await fs.exists(config.ffmpegRealname))) {
		await $${wgetPath} -nc  --no-check-certificate --show-progress ${config.windows.ffmpegUrl} -O ${config.windows.ffmpegName}.7z
		await $'C:\\Program Files\\7-Zip\\7z.exe' x ${config.windows.ffmpegName}.7z
		await $mv ${config.windows.ffmpegName} ${config.ffmpegRealname}
		await $rm -rf ${config.windows.ffmpegName}.7z
		await $mv ${config.ffmpegRealname}/lib/x64/* ${config.ffmpegRealname}/lib/
	}

	// Setup Tesseract
	const tesseractName = 'tesseract-setup'
	const tesseractUrl = 'https://github.com/UB-Mannheim/tesseract/releases/download/v5.4.0.20240606/tesseract-ocr-w64-setup-5.4.0.20240606.exe'
	const tesseractInstaller = ${tesseractName}.exe

	if (!(await fs.exists('tesseract'))) {
		console.log('Setting up Tesseract for Windows...')
		await $${wgetPath} -nc  --no-check-certificate --show-progress ${tesseractUrl} -O ${tesseractInstaller}
		await $"${process.cwd()}\\${tesseractInstaller}" /S /D=C:\\Program Files\\Tesseract-OCR
		await $rm ${tesseractInstaller}
		// Replace the mv command with xcopy
		await $xcopy "C:\\Program Files\\Tesseract-OCR" tesseract /E /I /H /Y
		// Optionally, remove the original directory if needed
		// await $rmdir "C:\\Program Files\\Tesseract-OCR" /S /Q
		console.log('Tesseract for Windows set up successfully.')
	} else {
		console.log('Tesseract for Windows already exists.')
	}

	// Add Tesseract to PATH
	process.env.PATH = ${process.cwd()}\\tesseract;${process.env.PATH}

	// Setup ONNX Runtime
	const onnxRuntimeName = "onnxruntime-win-x64-gpu-1.19.2";
	const onnxRuntimeLibs = ${onnxRuntimeName}.zip;
	const onnxRuntimeUrl = https://github.com/microsoft/onnxruntime/releases/download/v1.19.2/${onnxRuntimeLibs}
	if (!(await fs.exists(onnxRuntimeName))) {
		console.log('Setting up ONNX Runtime libraries for Windows...')
		await $${wgetPath} -nc  --no-check-certificate --show-progress ${onnxRuntimeUrl} -O ${onnxRuntimeLibs}
		await $unzip ${onnxRuntimeLibs} || tar -xf ${onnxRuntimeLibs} || echo "Done extracting";
		await $rm -rf ${onnxRuntimeLibs} || rm ${onnxRuntimeLibs} -Recurse -Force || echo "Done cleaning up zip";
		console.log('ONNX Runtime libraries for Windows set up successfully.')
	} else {
		console.log('ONNX Runtime libraries for Windows already exists.')
	}

	// Setup OpenBlas
	if (!(await fs.exists(config.openblasRealname)) && hasFeature('openblas')) {
		await $${wgetPath} -nc --show-progress ${config.windows.openBlasUrl} -O ${config.windows.openBlasName}.zip
		await $"C:\\Program Files\\7-Zip\\7z.exe" x ${config.windows.openBlasName}.zip -o${config.openblasRealname}
		await $rm ${config.windows.openBlasName}.zip
		fs.cp(path.join(config.openblasRealname, 'include'), path.join(config.openblasRealname, 'lib'), { recursive: true, force: true })
		// It tries to link only openblas.lib but our is libopenblas.lib
		fs.cp(path.join(config.openblasRealname, 'lib/libopenblas.lib'), path.join(config.openblasRealname, 'lib/openblas.lib'))
	}

	// Setup CLBlast
	if (!(await fs.exists(config.clblastRealname)) && !hasFeature('cuda')) {
		await $${wgetPath} -nc --show-progress ${config.windows.clblastUrl} -O ${config.windows.clblastName}.zip
		await $"C:\\Program Files\\7-Zip\\7z.exe" x ${config.windows.clblastName}.zip // 7z file inside
		await $"C:\\Program Files\\7-Zip\\7z.exe" x ${config.windows.clblastName}.7z // Inner folder
		await $mv ${config.windows.clblastName} ${config.clblastRealname}
		await $rm ${config.windows.clblastName}.zip
		await $rm ${config.windows.clblastName}.7z
	}

	// Setup vcpkg packages
	await $C:\\vcpkg\\vcpkg.exe install ${config.windows.vcpkgPackages}.quiet()
}

async function getMostRecentBinaryPath(targetArch, paths) {
	const validPaths = await Promise.all(paths.map(async (path) => {
		if (await fs.exists(path)) {
			const { stdout } = await $file ${path}.quiet();
			const binaryArch = stdout.includes('arm64') ? 'arm64' :
				stdout.includes('x86_64') ? 'x86_64' : null;
			if (binaryArch === targetArch) {
				const stat = await fs.stat(path);
				return { path, mtime: stat.mtime };
			}
		}
		return null;
	}));

	const filteredPaths = validPaths.filter(Boolean);

	if (filteredPaths.length === 0) {
		return null;
	}

	return filteredPaths.reduce((mostRecent, current) =>
		current.mtime > mostRecent.mtime ? current : mostRecent
	).path;
}
/* ########## macOS ########## */
if (platform == 'macos') {

	const architectures = ['arm64', 'x86_64'];

	for (const arch of architectures) {
		if (process.env['SKIP_SCREENPIPE_SETUP']) {
			break;
		}
		console.log(Setting up screenpipe bin for ${arch}...);

		if (arch === 'arm64') {
			const paths = [
				"../../target/aarch64-apple-darwin/release/screenpipe",
				"../../target/release/screenpipe"
			];

			const mostRecentPath = await getMostRecentBinaryPath('arm64', paths);
			if (mostRecentPath) {
				await $cp ${mostRecentPath} screenpipe-aarch64-apple-darwin;
				console.log(Copied most recent arm64 screenpipe binary from ${mostRecentPath});
			} else {
				console.error("No suitable arm64 screenpipe binary found");
			}
			// if the binary exists, hard code the fucking dylib
			if (await fs.exists('screenpipe-aarch64-apple-darwin') && !isDevMode) {
				await $install_name_tool -change screenpipe-vision/lib/libscreenpipe_arm64.dylib @rpath/../Frameworks/libscreenpipe_arm64.dylib ./screenpipe-aarch64-apple-darwin
				await $install_name_tool -change screenpipe-vision/lib/libscreenpipe.dylib @rpath/../Frameworks/libscreenpipe.dylib ./screenpipe-aarch64-apple-darwin
				console.log(hard coded the FUCKING dylib);
			} else if (await fs.exists('screenpipe-aarch64-apple-darwin') && isDevMode) {
				await $install_name_tool -change screenpipe-vision/lib/libscreenpipe_arm64.dylib @executable_path/../Frameworks/libscreenpipe_arm64.dylib ./screenpipe-aarch64-apple-darwin
				await $install_name_tool -change screenpipe-vision/lib/libscreenpipe.dylib @executable_path/../Frameworks/libscreenpipe.dylib ./screenpipe-aarch64-apple-darwin
				await $install_name_tool -add_rpath @executable_path/../Frameworks ./screenpipe-aarch64-apple-darwin
				console.log(Updated dylib paths for arm64 in dev mode);
			}


		} else if (arch === 'x86_64') {
			// copy screenpipe binary (more recent one)
			const paths = [
				"../../target/x86_64-apple-darwin/release/screenpipe",
				"../../target/release/screenpipe"
			];

			const mostRecentPath = await getMostRecentBinaryPath('x86_64', paths);

			if (mostRecentPath) {
				await $cp ${mostRecentPath} screenpipe-x86_64-apple-darwin;
				console.log(Copied most recent x86_64 screenpipe binary from ${mostRecentPath});
			} else {
				console.error("No suitable x86_64 screenpipe binary found");
			}
			// hard code the fucking dylib
			if (await fs.exists('screenpipe-x86_64-apple-darwin') && !isDevMode) {
				await $install_name_tool -change screenpipe-vision/lib/libscreenpipe_x86_64.dylib @rpath/../Frameworks/libscreenpipe_x86_64.dylib ./screenpipe-x86_64-apple-darwin
				await $install_name_tool -change screenpipe-vision/lib/libscreenpipe.dylib @rpath/../Frameworks/libscreenpipe.dylib ./screenpipe-x86_64-apple-darwin
				console.log(hard coded the FUCKING dylib);
			}

		}

		console.log(screenpipe for ${arch} set up successfully.);
	}


	// Setup FFMPEG
	if (!(await fs.exists(config.ffmpegRealname))) {
		await $wget -nc ${config.macos.ffmpegUrl} -O ${config.macos.ffmpegName}.tar.xz
		await $tar xf ${config.macos.ffmpegName}.tar.xz
		await $mv ${config.macos.ffmpegName} ${config.ffmpegRealname}
		await $rm ${config.macos.ffmpegName}.tar.xz
	} else {
		console.log('FFMPEG already exists');
	}

	// Move and rename ffmpeg and ffprobe binaries
	const ffmpegSrc = path.join(cwd, config.ffmpegRealname, 'bin', 'ffmpeg');

	// For x86_64
	await fs.copyFile(ffmpegSrc, path.join(cwd, 'ffmpeg-x86_64-apple-darwin'));

	// For arm64
	await fs.copyFile(ffmpegSrc, path.join(cwd, 'ffmpeg-aarch64-apple-darwin'));

	console.log('Moved and renamed ffmpeg binary for externalBin');
}

// Nvidia
let cudaPath
if (hasFeature('cuda')) {
	if (process.env['CUDA_PATH']) {
		cudaPath = process.env['CUDA_PATH']
	} else if (platform === 'windows') {
		const cudaRoot = 'C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\'
		cudaPath = 'C:\\Program Files\\NVIDIA GPU Computing Toolkit\\CUDA\\v12.5'
		if (await fs.exists(cudaRoot)) {
			const folders = await fs.readdir(cudaRoot)
			if (folders.length > 0) {
				cudaPath = cudaPath.replace('v12.5', folders[0])
			}
		}
	}

	if (process.env.GITHUB_ENV) {
		console.log('CUDA_PATH', cudaPath)
	}

	if (platform === 'windows') {
		const windowsConfig = {
			bundle: {
				resources: {
					'ffmpeg\\bin\\x64\\*': './',
					'openblas\\bin\\*.dll': './',
					[${cudaPath}\\bin\\cudart64_*]: './',
					[${cudaPath}\\bin\\cublas64_*]: './',
					[${cudaPath}\\bin\\cublasLt64_*]: './',
					'tesseract\\*': './',
					'onnxruntime*\\lib\\*.dll': './',
				},
				externalBin: [
					'screenpipe'
				]
			},
		}
		await fs.writeFile('tauri.windows.conf.json', JSON.stringify(windowsConfig, null, 4))
	}
	if (platform === 'linux') {
		// Add cuda toolkit depends package
		const tauriConfigContent = await fs.readFile('tauri.linux.conf.json', { encoding: 'utf-8' })
		const tauriConfig = JSON.parse(tauriConfigContent)
		tauriConfig.bundle.linux.deb.depends.push('nvidia-cuda-toolkit')
		await fs.writeFile('tauri.linux.conf.json', JSON.stringify(tauriConfig, null, 4))
	}
}

if (hasFeature('opencl')) {
	if (platform === 'windows') {
		const tauriConfigContent = await fs.readFile('tauri.windows.conf.json', { encoding: 'utf-8' })
		const tauriConfig = JSON.parse(tauriConfigContent)
		tauriConfig.bundle.resources['clblast\\bin\\*.dll'] = './'
		tauriConfig.bundle.resources['C:\\vcpkg\\packages\\opencl_x64-windows\\bin\\*.dll'] = './'
		await fs.writeFile('tauri.windows.conf.json', JSON.stringify(tauriConfig, null, 4))
	}
}

// OpenBlas
if (hasFeature('openblas')) {
	if (platform === 'windows') {
		const tauriConfigContent = await fs.readFile('tauri.windows.conf.json', { encoding: 'utf-8' })
		const tauriConfig = JSON.parse(tauriConfigContent)
		tauriConfig.bundle.resources['openblas\\bin\\*.dll'] = './'
		await fs.writeFile('tauri.windows.conf.json', JSON.stringify(tauriConfig, null, 4))
	}
}

// ROCM
let rocmPath = '/opt/rocm'
if (hasFeature('rocm')) {
	if (process.env.GITHUB_ENV) {
		console.log('ROCM_PATH', rocmPath)
	}
	if (platform === 'linux') {
		// Add rocm toolkit depends package
		const tauriConfigContent = await fs.readFile('tauri.linux.conf.json', { encoding: 'utf-8' })
		const tauriConfig = JSON.parse(tauriConfigContent)
		tauriConfig.bundle.linux.deb.depends.push('rocm')
		await fs.writeFile('tauri.linux.conf.json', JSON.stringify(tauriConfig, null, 4))
	}
}

// Development hints
if (!process.env.GITHUB_ENV) {
	console.log('\nCommands to build 🔨:')
	// Get relative path to screenpipe-app-tauri folder
	const relativePath = path.relative(originalCWD, path.join(cwd, '..'))
	if (originalCWD != cwd && relativePath != '') {
		console.log(cd ${relativePath})
	}
	console.log('bun install')
	if (platform == 'windows') {
		console.log($env:FFMPEG_DIR = "${exports.ffmpeg}")
		console.log($env:OPENBLAS_PATH = "${exports.openBlas}")
		console.log($env:LIBCLANG_PATH = "${exports.libClang}")
		console.log($env:PATH += "${exports.cmake}")
		if (hasFeature('older-cpu')) {
			console.log($env:WHISPER_NO_AVX = "ON")
			console.log($env:WHISPER_NO_AVX2 = "ON")
			console.log($env:WHISPER_NO_FMA = "ON")
			console.log($env:WHISPER_NO_F16C = "ON")
		}
		if (hasFeature('cuda')) {
			console.log($env:CUDA_PATH = "${cudaPath}")
		}
		if (hasFeature('opencl')) {
			console.log($env:CLBlast_DIR = "${exports.clblast}")
		}
		if (hasFeature('rocm')) {
			console.log($env:ROCM_VERSION = "6.1.2")
			console.log($env:ROCM_PATH = "${rocmPath}")
		}
	}
	if (!process.env.GITHUB_ENV) {
		console.log('bun tauri build')
	}
}

// Config Github ENV
if (process.env.GITHUB_ENV) {
	console.log('Adding ENV')
	if (platform == 'macos' || platform == 'windows') {
		const ffmpeg = FFMPEG_DIR=${exports.ffmpeg}\n
		console.log('Adding ENV', ffmpeg)
		await fs.appendFile(process.env.GITHUB_ENV, ffmpeg)
	}
	if (platform == 'macos') {
		const embed_metal = 'WHISPER_METAL_EMBED_LIBRARY=ON'
		await fs.appendFile(process.env.GITHUB_ENV, embed_metal)
	}
	if (platform == 'windows') {
		const openblas = OPENBLAS_PATH=${exports.openBlas}\n
		console.log('Adding ENV', openblas)
		await fs.appendFile(process.env.GITHUB_ENV, openblas)

		if (hasFeature('opencl')) {
			const clblast = CLBlast_DIR=${exports.clblast}\n
			console.log('Adding ENV', clblast)
			await fs.appendFile(process.env.GITHUB_ENV, clblast)
		}

		if (hasFeature('older-cpu')) {
			await fs.appendFile(process.env.GITHUB_ENV, WHISPER_NO_AVX=ON\n)
			await fs.appendFile(process.env.GITHUB_ENV, WHISPER_NO_AVX2=ON\n)
			await fs.appendFile(process.env.GITHUB_ENV, WHISPER_NO_FMA=ON\n)
			await fs.appendFile(process.env.GITHUB_ENV, WHISPER_NO_F16C=ON\n)
		}
	}
}

// --dev or --build
const action = process.argv?.[2]
if (action?.includes('--build' || action.includes('--dev'))) {
	process.chdir(path.join(cwd, '..'))
	process.env['FFMPEG_DIR'] = exports.ffmpeg
	if (platform === 'windows') {
		process.env['OPENBLAS_PATH'] = exports.openBlas
		process.env['CLBlast_DIR'] = exports.clblast
		process.env['LIBCLANG_PATH'] = exports.libClang
		process.env['PATH'] = ${process.env['PATH']};${exports.cmake}
	}
	if (platform == 'macos') {
		process.env['WHISPER_METAL_EMBED_LIBRARY'] = 'ON'
	}
	await $bun install
	await $bunx tauri ${action.includes('--dev') ? 'dev' : 'build'}
}




will it be an issue using node.js instead of bun?

2/2

ChatGPT

Thought for 23 seconds

Yes, using Node.js instead of bun to run the pre_build.js script will require some modifications, but it's entirely feasible. The script you've provided is tailored for bun, which has certain features and built-in functions that aren't available in Node.js by default. However, with some adjustments, you can adapt the script to work with Node.js on your Windows ARM64 machine.

Understanding the Differences Between bun and Node.js
Before diving into the modifications, it's essential to understand the key differences between bun and Node.js that affect your script:

Shell Command Execution:

bun: Provides a built-in $ function for executing shell commands.
Node.js: Doesn't have a built-in $ function.
Module Imports:

bun: Supports ES module syntax with top-level await by default.
Node.js: Requires enabling ES modules and may need adjustments for top-level await.
File System and Other APIs:

Most APIs like fs, os, and path are similar, but there might be slight differences.
Steps to Modify pre_build.js for Node.js
1. Replace bun's $ Function
The $ function in bun allows for easy execution of shell commands. In Node.js, you can achieve similar functionality using packages or the built-in child_process module.

Option A: Use child_process

Import exec and execSync:

javascript
Copy code
const { exec, execSync } = require('child_process');
Replace $ Calls:

javascript
Copy code
await $`command`;
Becomes:

javascript
Copy code
execSync('command', { stdio: 'inherit' });
Option B: Use the zx Library (Recommended)

Google's zx library brings a scripting experience similar to bun's $ function.

Install zx:

bash
Copy code
npm install zx
Update Shebang and Imports:

At the top of your script, add:

javascript
Copy code
#!/usr/bin/env zx
Replace:

javascript
Copy code
import { $ } from 'bun';
With:

javascript
Copy code
import 'zx/globals';
**Now, $ will be available in your script similar to bun.

2. Adjust Module Imports
Since Node.js doesn't support ES modules by default without additional configuration, you have two options:

Option A: Convert to CommonJS Syntax

Replace import with require:

javascript
Copy code
// From:
import fs from 'fs/promises';
import os from 'os';
import path from 'path';

// To:
const fs = require('fs').promises;
const os = require('os');
const path = require('path');
Option B: Enable ES Modules in Node.js

Add "type": "module" to package.json:

json
Copy code
{
  "type": "module",
  "dependencies": {
    "zx": "^7.1.1"
  }
}
Ensure your script files use the .mjs extension or are run with the --experimental-modules flag.

3. Handle Top-Level await
Node.js supports top-level await in ES modules (with "type": "module" in package.json). If you opt for CommonJS, wrap your asynchronous code in an async function.

Example:

javascript
Copy code
(async () => {
  // Your code here
})();
4. Modify Shell Commands for Cross-Platform Compatibility
Since you're on Windows, some shell commands used in the script (mv, rm, cp, etc.) aren't available. You need to replace them with Node.js equivalents or use a cross-platform library.

Option A: Use Node.js fs Module

Replace mv (move) Command:

javascript
Copy code
// From:
await $`mv source destination`;

// To:
await fs.rename('source', 'destination');
Replace rm (remove) Command:

javascript
Copy code
// From:
await $`rm file`;

// To:
await fs.unlink('file');
Replace cp (copy) Command:

javascript
Copy code
// From:
await $`cp source destination`;

// To:
await fs.copyFile('source', 'destination');
Option B: Use the shelljs Library

Install shelljs:

bash
Copy code
npm install shelljs
Import and Use:

javascript
Copy code
const shell = require('shelljs');

shell.mv('source', 'destination');
shell.rm('file');
shell.cp('source', 'destination');
5. Update Platform-Specific Paths and Commands
Ensure that any file paths and commands are adjusted for Windows syntax.

Example:

Replace:

javascript
Copy code
const wgetPath = await findWget();
With:

Ensure wget is installed or use PowerShell's Invoke-WebRequest.

6. Review and Update Environment Variables
Environment variables might need to be set differently in Node.js scripts.

Setting Environment Variables in Node.js:

javascript
Copy code
process.env.FFMPEG_DIR = exports.ffmpeg;
Appending to PATH:

javascript
Copy code
process.env.PATH = `${process.env.PATH};${exports.cmake}`;
Example Modified Script
Here's how parts of your script might look after modifications:

javascript
Copy code
#!/usr/bin/env zx

import 'zx/globals';
import fs from 'fs/promises';
import os from 'os';
import path from 'path';

const isDevMode = process.env.SCREENPIPE_APP_DEV === 'true' || false;

const originalCWD = process.cwd();
// Change CWD to src-tauri
process.chdir(path.join(__dirname, '../src-tauri'));
const platform = {
  win32: 'windows',
  darwin: 'macos',
  linux: 'linux',
}[os.platform()];
const cwd = process.cwd();
console.log('cwd', cwd);
function hasFeature(name) {
  return process.argv.includes(`--${name}`) || process.argv.includes(name);
}

// Rest of your script...

// Example of replacing a shell command:

// From bun's $ (assuming using zx now):
await $`wget -nc ${config.macos.ffmpegUrl} -O ${config.macos.ffmpegName}.tar.xz`;

// To use Node.js fetch (if possible) or an alternative method to download files on Windows.
Potential Issues and Solutions
1. Downloading Files
wget may not be available on Windows. You can use Node.js packages like node-fetch or built-in modules to download files.

Using node-fetch:

Install:

bash
Copy code
npm install node-fetch
Import and Use:

javascript
Copy code
import fetch from 'node-fetch';
import { createWriteStream } from 'fs';

const response = await fetch(fileUrl);
const fileStream = createWriteStream(destinationPath);
await new Promise((resolve, reject) => {
  response.body.pipe(fileStream);
  response.body.on('error', reject);
  fileStream.on('finish', resolve);
});
2. Extracting Archives
For extracting .zip or .7z files, use libraries like adm-zip or node-7z.

Install:

bash
Copy code
npm install adm-zip
Use:

javascript
Copy code
import AdmZip from 'adm-zip';

const zip = new AdmZip('file.zip');
zip.extractAllTo('destination_folder', true);
3. Handling Asynchronous Operations
Ensure all asynchronous operations use await if you're using top-level await, or manage them within async functions.

4. Cross-Platform Paths
Use path.join and path.resolve to construct file paths, ensuring they are compatible with Windows.

Final Considerations
Testing: After making these changes, thoroughly test the script to ensure all parts are working as expected.
Dependencies: Ensure all required Node.js packages are installed and included in your package.json.
Community Support: If you encounter specific errors, consider reaching out to the Screenpipe community or maintainers for assistance.
Conclusion
Switching from bun to Node.js will require some effort, mainly due to the differences in shell command execution and module imports. However, by:

Replacing bun's $ function with Node.js alternatives like zx or child_process.
Adjusting module imports and handling asynchronous code appropriately.
Modifying shell commands for Windows compatibility.
Using Node.js packages to handle tasks like downloading and extracting files.
You can successfully run the pre_build.js script using Node.js on your Windows ARM64 machine.

Feel free to ask if you need further clarification on any of these steps or assistance with specific parts of the script.






