async function drawRadar() {
  const data = await fetch("./radarEntries.json");
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
      { name: "Web Development" },
      { name: "Database and Data Storage" },
      { name: "Machine Learning and Data Science" },
      { name: "DevOps and Cloud Services" },
    ],
    rings: [
      { name: "Adopt", color: "#5ba300" },
      { name: "Trial", color: "#009eb0" },
      { name: "Assess", color: "#c7ba00" },
      { name: "Hold", color: "#e09b96" },
    ],
    print_layout: true,
    links_in_new_tabs: true,
    entries: [
      {
        label: "Python",
        quadrant: 2, // 0,1,2,3 (counting clockwise, starting from bottom right)
        ring: 0, // 0,1,2,3 (starting from inside)
        moved: 0, // -1 = moved out (triangle pointing down)
        //  0 = not moved (circle)
        //  1 = moved in  (triangle pointing up)
      },
      {
        label: "scikit-learn",
        quadrant: 2,
        ring: 0,
        moved: 0,
      },
      {
        label: "PySpark",
        quadrant: 2,
        ring: 0,
        moved: 0,
      },
      {
        label: "Pandas",
        quadrant: 2,
        ring: 0,
        moved: 0,
      },
      {
        label: "xgboost",
        quadrant: 2,
        ring: 0,
        moved: 0,
      },
      {
        label: "statsmodels",
        quadrant: 2,
        ring: 0,
        moved: 0,
      },
      {
        label: "Langchain",
        quadrant: 2,
        ring: 0,
        moved: 0,
      },
      {
        label: "SQL",
        quadrant: 2,
        ring: 0,
        moved: 0,
      },
      {
        label: "scala",
        quadrant: 2,
        ring: 2,
        moved: 0,
      },
      {
        label: "s3",
        quadrant: 1,
        ring: 0,
        moved: 0,
      },
      {
        label: "PostgreSQL",
        quadrant: 1,
        ring: 0,
        moved: 0,
      },
      {
        label: "SQLAlchemy",
        quadrant: 1,
        ring: 0,
        moved: 0,
      },
      {
        label: "Google Storage Bucket",
        quadrant: 1,
        ring: 0,
        moved: 0,
      },
      {
        label: "Firestore",
        quadrant: 1,
        ring: 0,
        moved: 0,
      },
      {
        label: "Alembic",
        quadrant: 1,
        ring: 0,
        moved: 0,
      },
      {
        label: "Redis",
        quadrant: 1,
        ring: 0,
        moved: 0,
      },
      {
        label: "Typescript",
        quadrant: 0,
        ring: 0,
        moved: 0,
      },
      {
        label: "Javascript",
        quadrant: 0,
        ring: 3,
        moved: 0,
      },
      {
        label: "Axum",
        quadrant: 0,
        ring: 1,
        moved: 0,
      },
      {
        label: "Rust",
        quadrant: 0,
        ring: 1,
        moved: 0,
      },
      {
        label: "Flask",
        quadrant: 0,
        ring: 3,
        moved: 0,
      },
      {
        label: "FastAPI",
        quadrant: 0,
        ring: 0,
        moved: 0,
      },
      {
        label: "Django",
        quadrant: 0,
        ring: 2,
        moved: 0,
      },
      {
        label: "React",
        quadrant: 0,
        ring: 0,
        moved: 0,
      },
      {
        label: "NextJS",
        quadrant: 0,
        ring: 0,
        moved: 0,
      },
      {
        label: "Vue",
        quadrant: 0,
        ring: 2,
        moved: 0,
      },
      {
        label: "HTMX",
        quadrant: 0,
        ring: 2,
        moved: 0,
      },
      {
        label: "TailwindCSS",
        quadrant: 0,
        ring: 2,
        moved: 0,
      },
      {
        label: "OpenAPI",
        quadrant: 0,
        ring: 0,
        moved: 0,
      },
    ],
  });
}

document.addEventListener("DOMContentLoaded", () => {
  drawRadar();
});
