<template>
  <p v-if="msg.length">{{ msg }}</p>
  <p v-if="errorMsg.length" style="color: red">{{ errorMsg }}</p>
  <Keyboard />


</template>

<script setup lang="ts">
import { ref, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import { save } from '@tauri-apps/api/dialog';
import { writeBinaryFile } from '@tauri-apps/api/fs';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import Keyboard from './Keyboard.vue';
import Button from './Button.vue';

const msg = ref("");
const errorMsg = ref("");
const buttonText = ref("Run");

const unlisten = listen('file', async (event: any) => {
  console.log(event)
  if (event.payload.success) {
    // const buffer = event.payload.data as Uint8Array
    // const filePath = await save({
    //   filters: [{
    //     name: "DTMF",
    //     extensions: ['wav']
    //   }]
    // });
    // if (filePath) {
    //   await writeBinaryFile(filePath, buffer);
    // }
  } else {
    errorMsg.value = '获取文件失败'
  }
})

onUnmounted(async () => {
  (await unlisten)()
})


async function run() {
  buttonText.value = "Running..."
  try {
    errorMsg.value = ""

    let res = await invoke("run_python_script", {
      script: `
import numpy as np
import wave
import tempfile
import dtmf

SAMPLE_RATE = 44100

def generate_dtmf_tone(digits):
    tone_duration = 0.3
    silence_duration = 0.2
    dtmf_frequencies = {
        '1': (697, 1209),
        '2': (697, 1336),
        '3': (697, 1477),
        'A': (697, 1633),
        '4': (770, 1209),
        '5': (770, 1336),
        '6': (770, 1477),
        'B': (770, 1633),
        '7': (852, 1209),
        '8': (852, 1336),
        '9': (852, 1477),
        'C': (852, 1633),
        '*': (941, 1209),
        '0': (941, 1336),
        '#': (941, 1477),
        'D': (941, 1633),
    }

    if len(digits) == 1:
        silence_duration = 0

    signal = np.zeros(int(silence_duration * SAMPLE_RATE))
    time_points = np.arange(0, int(tone_duration * SAMPLE_RATE))

    for digit in digits:
        low_freq, high_freq = dtmf_frequencies[digit]
        tone = np.sin(2 * np.pi * low_freq * time_points / SAMPLE_RATE) + \
              np.sin(2 * np.pi * high_freq * time_points / SAMPLE_RATE)
        silence_gap = np.zeros(int(silence_duration * SAMPLE_RATE))
        signal = np.concatenate((signal, tone, silence_gap))

    return signal

def save_wave(file_name, audio_data, sample_rate, sample_width=2, num_channels=1):
    wave_file = wave.open(file_name, "wb")
    wave_file.setnchannels(num_channels)
    wave_file.setsampwidth(sample_width)
    wave_file.setframerate(sample_rate)

    audio_data = audio_data * int(20000 / np.max(np.abs(audio_data)))
    audio_data = np.int16(audio_data)
    wave_file.writeframes(audio_data.tobytes())
    wave_file.close()


dtmf_signal = generate_dtmf_tone('1234567890*#ABCD')
filename = tempfile.mktemp(suffix='.wav')
save_wave(filename, dtmf_signal, SAMPLE_RATE)
dtmf.submit(filename)
  `})
    if (!res) {
      msg.value = 'Success!'
    } else {
      errorMsg.value = (res as string).toString()
    }
  } catch (e) {
    errorMsg.value = (e as Error).toString() || "An error occurred"
  } finally {
    buttonText.value = "Run"
  }
}
</script>