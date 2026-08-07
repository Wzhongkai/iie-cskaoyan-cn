<script lang="ts">
  import { onMount } from 'svelte';
  import type { ECharts, EChartsCoreOption as EChartsOption } from 'echarts/core';

  let { option, height = 340, label = '数据图表' }: { option: EChartsOption; height?: number; label?: string } = $props();
  let container: HTMLDivElement;
  let chart: ECharts | undefined;
  let latestOption: EChartsOption | undefined;

  function applyOption(nextOption: EChartsOption) {
    latestOption = nextOption;
    chart?.setOption(nextOption, { notMerge: true, lazyUpdate: false });
  }

  onMount(() => {
    let observer: ResizeObserver | undefined;
    let disposed = false;
    import('$lib/chart-runtime').then(({ echarts }) => {
      if (disposed) return;
      chart = echarts.init(container, undefined, { renderer: 'canvas' });
      applyOption(latestOption ?? option);
      observer = new ResizeObserver(() => chart?.resize());
      observer.observe(container);
    });
    return () => {
      disposed = true;
      observer?.disconnect();
      chart?.dispose();
    };
  });

  $effect(() => {
    applyOption(option);
  });
</script>

<div class="chart" bind:this={container} style={`height: ${height}px`} role="img" aria-label={label}></div>

<style>
  .chart { width: 100%; min-width: 0; }
</style>
