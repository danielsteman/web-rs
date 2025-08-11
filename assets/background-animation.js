import * as THREE from "https://cdn.skypack.dev/three@0.148.0";

export function initBackgroundAnimation(containerId = "canvas-container") {
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

  const fragmentShaderSource = `
precision highp float;

uniform float u_time;
uniform vec2 u_resolution;

float rand(vec2 co) {
  return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453);
}

float blob(vec2 uv, vec2 center, float size, float speed, float t) {
  float d = length(uv - center + 0.05 * sin(t * speed));
  return smoothstep(size, size - 0.1, d);
}

void main() {
  vec2 uv = gl_FragCoord.xy / u_resolution.xy;
  float t = u_time * 0.3;

  float wave1 = sin(uv.x * 6.0 + t * 1.0) * 0.6;
  float wave2 = cos(uv.y * 8.0 - t * 1.2) * 0.5;
  float wave3 = sin((uv.x + uv.y) * 4.0 + t * 0.8) * 0.4;

  float gradientMix = clamp(uv.y + wave1 + wave2 + wave3, 0.0, 1.0);

  vec3 colorA = vec3(0.95, 0.7, 0.95);
  vec3 colorB = vec3(1.0, 0.95, 0.75);

  vec3 base = mix(colorA, colorB, gradientMix);

  float blob1 = blob(uv, vec2(0.5, 0.15), 0.8, 0.08, u_time);
  float blob2 = blob(uv, vec2(0.5, 0.85), 0.8, 0.06, u_time + 200.0);
  float shapeMask = clamp(blob1 + blob2, 0.0, 1.0);

  vec3 fogColor = vec3(0.85, 0.75, 0.95);
  vec3 withShapes = mix(base, fogColor, shapeMask * 0.65);

  float noise = rand(uv * u_resolution.xy * 0.1 + u_time * 5.0) * 0.2;

  gl_FragColor = vec4(withShapes + noise, 1.0);
}
`;

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
