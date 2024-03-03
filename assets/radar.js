async function drawRadar() {
  const data = await fetch("assets/radar.json");
  const jsonData = await data.json();
  radar_visualization({
    svg_id: "radar",
    width: 1450,
    height: 1000,
    scale: 1,
    colors: {
      background: "#f3f4f6",
      grid: "#111827",
      inactive: "#ddd",
    },
    // title: "Personal Tech Radar",
    quadrants: [
      { name: "Web Development" },
      { name: "Database and Data Storage" },
      { name: "Machine Learning and Data Science" },
      { name: "DevOps and Cloud Services" },
    ],
    rings: [
      { name: "Adopt", color: "#a855f7" },
      { name: "Trial", color: "#6366f1" },
      { name: "Assess", color: "#ec4899" },
      { name: "Hold", color: "#0ea5e9" },
    ],
    print_layout: true,
    links_in_new_tabs: true,
    entries: jsonData.entries,
  });
}

document.addEventListener("DOMContentLoaded", () => {
  drawRadar();
});
