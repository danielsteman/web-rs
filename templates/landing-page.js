import * as THREE from 'https://cdn.skypack.dev/three@0.148.0';

const scene = new THREE.Scene();
const camera = new THREE.Camera();

const renderer = new THREE.WebGLRenderer({ antialias: true });
renderer.setPixelRatio(window.devicePixelRatio); // High-DPI support
renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

// Uniforms
const uniforms = {
  u_time: { value: 0.0 },
  u_resolution: { value: new THREE.Vector2(window.innerWidth, window.innerHeight) }
};

// Shader Material
const material = new THREE.ShaderMaterial({
  uniforms,
  fragmentShader: `
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
      float t = u_time * 0.3; // faster time for gradient motion

      // More intense wave distortion
      float wave1 = sin(uv.x * 6.0 + t * 1.2) * 0.4;
      float wave2 = cos(uv.y * 8.0 - t * 1.5) * 0.3;
      float wave3 = sin((uv.x + uv.y) * 4.0 + t * 0.8) * 0.2;
  
      float gradientMix = clamp(uv.y + wave1 + wave2 + wave3, 0.0, 1.0);
  
      // Brighter and more contrasty color endpoints
      vec3 colorA = vec3(0.95, 0.7, 0.95); // bright violet pink
      vec3 colorB = vec3(1.0, 0.95, 0.75); // warm pastel yellow
  
      vec3 base = mix(colorA, colorB, gradientMix);

      // Moving soft blob shapes
      float blob1 = blob(uv, vec2(0.3, 0.4), 0.5, 0.25, u_time);
      float blob2 = blob(uv, vec2(0.7, 0.6), 0.4, 0.2, u_time);
      float blob3 = blob(uv, vec2(0.5, 0.5), 0.6, 0.15, u_time + 100.0);
      float shapeMask = clamp(blob1 + blob2 + blob3, 0.0, 1.0);

      vec3 fogColor = vec3(0.85, 0.75, 0.95); // pastel lavender with more blue
      vec3 withShapes = mix(base, fogColor, shapeMask * 0.45);

      // Grain noise
      float noise = rand(uv * u_resolution.xy * 0.1 + u_time * 5.0) * 0.12;

      gl_FragColor = vec4(withShapes + noise, 1.0);
    }
  `
});

// Fullscreen quad
const plane = new THREE.PlaneGeometry(2, 2);
const quad = new THREE.Mesh(plane, material);
scene.add(quad);

// Resize handler
window.addEventListener('resize', () => {
  const width = window.innerWidth;
  const height = window.innerHeight;
  renderer.setSize(width, height);
  uniforms.u_resolution.value.set(width, height);
});

// Animation loop
function animate(time) {
  uniforms.u_time.value = time * 0.001;
  renderer.render(scene, camera);
  requestAnimationFrame(animate);
}
animate();

