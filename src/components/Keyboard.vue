<template>
    <div class="h-[calc(100vh-304px)] w-full bg-white dark:bg-gray-900 text-black dark:text-white text-center text-4xl flex items-center justify-center break-words whitespace-normal break-all px-5 py-10 overflow-y-scroll">
        <span v-if="value">{{ value }}</span>
        <span v-else class="text-gray-400 text-xl">使用按钮或键盘输入</span>
    </div>
    <div class="h-6 text-center text-gray-400 text-xs pt-2 bg-white dark:bg-gray-900 dark:text-gray-600">
        © 
        <a href="https://github.com/youxam" target="_blank">YouXam</a>
    </div> 
    <div class="grid grid-cols-4 gap-0">
        <Button @click="append('1')">
            <FontAwesomeIcon :icon="['fass', '1']" />
        </Button>
        <Button @click="append('2')">
            <FontAwesomeIcon :icon="['fass', '2']" />
        </Button>
        <Button @click="append('3')">
            <FontAwesomeIcon :icon="['fass', '3']" />
        </Button>
        <Button @click="append('A')">
            <FontAwesomeIcon :icon="['fas', 'a']" />
        </Button>
        <Button @click="append('4')">
            <FontAwesomeIcon :icon="['fass', '4']" />
        </Button>
        <Button @click="append('5')">
            <FontAwesomeIcon :icon="['fass', '5']" />
        </Button>
        <Button @click="append('6')">
            <FontAwesomeIcon :icon="['fass', '6']" />
        </Button>
        <Button @click="append('B')">
            <FontAwesomeIcon :icon="['fas', 'b']" />
        </Button>
        <Button @click="append('7')">
            <FontAwesomeIcon :icon="['fass', '7']" />
        </Button>
        <Button @click="append('8')">
            <FontAwesomeIcon :icon="['fass', '8']" />
        </Button>
        <Button @click="append('9')">
            <FontAwesomeIcon :icon="['fass', '9']" />
        </Button>
        <Button @click="append('C')">
            <FontAwesomeIcon :icon="['fas', 'c']" />
        </Button>
        <Button @click="append('*')">
            <FontAwesomeIcon :icon="['fass', 'asterisk']" />
        </Button>
        <Button @click="append('0')">
            <FontAwesomeIcon :icon="['fass', '0']" />
        </Button>
        <Button @click="append('#')">
            <FontAwesomeIcon :icon="['fass', 'hashtag']" />
        </Button>
        <Button @click="append('D')">
            <FontAwesomeIcon :icon="['fas', 'd']" />
        </Button>
    </div>
    <div class="grid grid-cols-3">
        <button
            @click="value = value.slice(0, -1)"
            class="font-medium py-4 px-6 w-full space-x-3 bg-[#743ee4] text-white hover:bg-[#8150e6] active:bg-[#6a3ad9] focus:outline-none focus:ring focus:ring-[#8150e6]">
            <FontAwesomeIcon :icon="['fas', 'delete-left']" />
            <span>
                退格
            </span>
        </button>
        <button
            @click="clear"
            class="font-medium py-4 px-6 w-full space-x-3 bg-[#ff4d4f] text-white hover:bg-[#ff5b5d] active:bg-[#e63a3c] focus:outline-none focus:ring focus:ring-[#ff5b5d]">
            <FontAwesomeIcon :icon="['fas', 'trash']" />
            <span>
                清空
            </span>
        </button>

        <button
            @click="saveFile"
            class="w-full space-x-3 bg-[#3662e3] text-white hover:bg-[#4871e5] active:bg-[#2a4bba] focus:outline-none focus:ring focus:ring-[#4871e5]">
            <FontAwesomeIcon :icon="['fas', 'file-arrow-down']" />
            <span>
                保存
            </span>
        </button>
    </div>
</template>

<script setup lang="ts">
import { ref, onUnmounted } from 'vue'
import Button from './Button.vue'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { save } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api/tauri';
const value = ref('')
function clear() {
    value.value = ''
}

async function playWav(wav: ArrayBuffer) {
    try {
        // @ts-ignore
        const audioContext = new (window.AudioContext || window.webkitAudioContext)();
        const audioBuffer = await audioContext.decodeAudioData(wav);
        const source = audioContext.createBufferSource();
        source.buffer = audioBuffer;
        source.connect(audioContext.destination);
        source.start();
    } catch (error) {
        console.error('Error with decoding audio data', error);
    }
}
async function play(v: string) {
    try {
        const wav: number[] = await invoke('tone', { char: v })
        const wavBuffer = new Uint8Array(wav)
        await playWav(wavBuffer.buffer)
    } catch (e) {
        console.error(e)
    }
}
function append(v: string) {
    value.value += v
    play(v)
}
async function saveFile() {
    const filePath = await save({
        filters: [{
            name: "DTMF",
            extensions: ['wav']
        }]
    })
    await invoke('save', { path: filePath, value: value.value })
}
function hanlder(e: KeyboardEvent) {
    const key = e.key
    if (['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '*', '#', 'A', 'B', 'C', 'D', 'a', 'b', 'c', 'd'].includes(key)) {
        append(key.toUpperCase())
    } else if (key === 'Backspace') {
        value.value = value.value.slice(0, -1)
    } else if (key === 'Escape') {
        clear()
    }
}

document.addEventListener('keydown', hanlder)
onUnmounted(() => {
    document.removeEventListener('keydown', hanlder)
})
</script>