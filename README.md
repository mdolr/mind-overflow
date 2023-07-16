# mind-overflow

Open-source [Rewind.AI](https://rewind.ai) clone built with [Tauri](https://tauri.app) and Vue. Leverages [whisper.cpp](https://github.com/ggerganov/whisper.cpp) for Speech-to-Text and (wip: [llama.cpp](https://github.com/ggerganov/llama.cpp) for Text generation and Question Answering using Orca 13B.)

Inspired from

- [Rewind.AI](https://rewind.ai)
- [Talk](https://github.com/yacineMTB/talk)
- [Cyte](https://github.com/dhamaniasad/cytev2)

## tl;dr

This software captures all textual content displayed on your screen or heard through your microphone and turns it into a searchable and interactive database. Allowing you to go back at any point in time to find what you were looking for in its original context.

## Models

Still WIP for now download GGML base.bin whisper.cpp model using instructions in [whisper.cpp](https://github.com/ggerganov/whisper.cpp/tree/master/models) then place it in `src-tauri/assets/models/ggml-small.bin`

## License

The rest of the code is licensed under the MIT License
