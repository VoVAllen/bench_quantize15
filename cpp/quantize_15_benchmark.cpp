#include <benchmark/benchmark.h>
#include <vector>
#include <random>
#include <algorithm>

// Include the quantize_15_optimized function here
// ... [previous quantize_15_optimized function code] ...
#include <vector>
#include <tuple>
#include <limits>
#include <algorithm>

std::tuple<float, float, std::vector<uint8_t>> quantize_15_optimized(const std::vector<float>& lut) {
    if (lut.empty()) {
        return {0.0f, 0.0f, std::vector<uint8_t>()};
    }

    auto [min_it, max_it] = std::minmax_element(lut.begin(), lut.end());
    float min = *min_it;
    float max = *max_it;

    float k = std::max(max - min, 0.0f) / 15.0f;
    float b = min;

    std::vector<uint8_t> quantized;
    quantized.reserve(lut.size());

    if (k == 0.0f) {
        quantized.assign(lut.size(), 0);
    } else {
        float scale = 15.0f / (max - min);
        for (float y : lut) {
            quantized.push_back(static_cast<uint8_t>((y - min) * scale));
        }
    }

    return {k, b, quantized};
}
std::vector<float> generate_random_lut(size_t size) {
    std::vector<float> lut(size);
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_real_distribution<> dis(-1000.0, 1000.0);

    std::generate(lut.begin(), lut.end(), [&]() { return dis(gen); });
    return lut;
}

static void BM_Quantize15Small(benchmark::State& state) {
    auto lut = generate_random_lut(100);
    for (auto _ : state) {
        auto result = quantize_15_optimized(lut);
        benchmark::DoNotOptimize(result);
    }
}

static void BM_Quantize15Medium(benchmark::State& state) {
    auto lut = generate_random_lut(10000);
    for (auto _ : state) {
        auto result = quantize_15_optimized(lut);
        benchmark::DoNotOptimize(result);
    }
}

static void BM_Quantize15Large(benchmark::State& state) {
    auto lut = generate_random_lut(1000000);
    for (auto _ : state) {
        auto result = quantize_15_optimized(lut);
        benchmark::DoNotOptimize(result);
    }
}

BENCHMARK(BM_Quantize15Small);
BENCHMARK(BM_Quantize15Medium);
BENCHMARK(BM_Quantize15Large);

BENCHMARK_MAIN();