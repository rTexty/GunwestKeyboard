import wave
import struct
import math
import random

# Parameters
sample_rate = 44100

def generate_tone(filename, frequency, duration, decay_speed=100.0):
    samples = []
    # Generate audio samples
    num_samples = int(sample_rate * duration)
    for i in range(num_samples):
        t = float(i) / sample_rate
        # Simple decaying sine wave
        envelope = math.exp(-t * decay_speed) 
        value = int(32767.0 * envelope * math.sin(2.0 * math.pi * frequency * t))
        samples.append(value)

    # Write to WAV file
    with wave.open(filename, 'w') as wav_file:
        wav_file.setnchannels(1) # Mono
        wav_file.setsampwidth(2) # 16-bit
        wav_file.setframerate(sample_rate)
        for sample in samples:
            # Struct pack as little-endian short
            wav_file.writeframes(struct.pack('<h', sample))
    print(f"Generated {filename}")

# 1. Generate 4 clicks (slightly different frequencies)
for i in range(1, 5):
    freq = 800.0 + (i * 150.0) # 950, 1100, 1250, 1400 Hz
    generate_tone(f"click{i}.wav", freq, 0.05, 100.0)

# 2. Generate Enter (Loud, deeper, longer)
generate_tone("enter.wav", 400.0, 0.15, 60.0)

# 3. Generate Cmd (High pitch, sharp)
generate_tone("cmd.wav", 1800.0, 0.05, 150.0)
