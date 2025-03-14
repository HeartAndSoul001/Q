import { createRouter, createWebHistory } from 'vue-router';
import SubnetsCalcu from '../components/iptools/subnetsCalcu.vue';

const routes = [
  { path: '/', component: SubnetsCalcu },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;