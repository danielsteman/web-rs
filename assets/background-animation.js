import * as THREE from "https://cdn.skypack.dev/three@0.148.0";

export async function initBackgroundAnimation(
  containerId = "canvas-container"
) {
  const scene = new THREE.Scene();
  const camera = new THREE.Camera();

  const renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setPixelRatio(window.devicePixelRatio);
  renderer.setSize(window.innerWidth, window.innerHeight);
  renderer.domElement.classList.add("absolute", "top-0", "left-0", "z-0");

  const container = document.getElementById(containerId);
  if (container) {
    container.appendChild(renderer.domElement);
  }

  const fragmentShaderSource = await fetch(
    "/assets/shaders/background.frag"
  ).then((res) => res.text());

  const uniforms = {
    u_time: { value: 0.0 },
    u_resolution: {
      value: new THREE.Vector2(window.innerWidth, window.innerHeight),
    },
  };

  const material = new THREE.ShaderMaterial({
    uniforms,
    fragmentShader: fragmentShaderSource,
  });

  const plane = new THREE.PlaneGeometry(2, 2);
  const quad = new THREE.Mesh(plane, material);
  scene.add(quad);

  window.addEventListener("resize", () => {
    const width = window.innerWidth;
    const height = window.innerHeight;
    renderer.setSize(width, height);
    uniforms.u_resolution.value.set(width, height);
  });

  function animate(time) {
    uniforms.u_time.value = time * 0.001;
    renderer.render(scene, camera);
    requestAnimationFrame(animate);
  }

  animate();
}
