async function drawRadar() {
  radar_visualization({
    svg_id: "radar",
    width: 1450,
    height: 1000,
    scale: 1.0,
    colors: {
      background: "#fff",
      grid: "#bbb",
      inactive: "#ddd",
    },
    title: "My Radar",
    quadrants: [
      { name: "Bottom Right" },
      { name: "Bottom Left" },
      { name: "Top Left" },
      { name: "Top Right" },
    ],
    rings: [
      { name: "INNER", color: "#5ba300" },
      { name: "SECOND", color: "#009eb0" },
      { name: "THIRD", color: "#c7ba00" },
      { name: "OUTER", color: "#e09b96" },
    ],
    print_layout: true,
    links_in_new_tabs: true,
    entries: [
      {
        label: "Some Entry",
        quadrant: 3, // 0,1,2,3 (counting clockwise, starting from bottom right)
        ring: 2, // 0,1,2,3 (starting from inside)
        moved: -1, // -1 = moved out (triangle pointing down)
        //  0 = not moved (circle)
        //  1 = moved in  (triangle pointing up)
      },
      // ...
    ],
  });
}

document.addEventListener("DOMContentLoaded", () => {
  drawRadar();
});
