import Vue from 'vue'
import VueRouter, { RouteConfig } from 'vue-router'

Vue.use(VueRouter)

const routes: Array<RouteConfig> = [
  {
    path: '/',
    component: () => import('../views/album/AlbumGridView.vue')
  },
  {
    path: '/album/view',
    component: () => import('../views/album/AlbumDetailView.vue'),
  },
  {
    path: '/album/create',
    component: () => import('../views/album/AlbumCreateView.vue'),
  },
  {
    path: '/album/edit',
    component: () => import('../views/album/AlbumEditView.vue'),
  },
  {
    path: '/photo/view',
    component: () => import('../views/photo/PhotoDetailView.vue'),
  },
  {
    path: '/logged_in',
    component: () => import('../views/LoggedIn.vue'),
  },
  {
    path: '/user',
    component: () => import('../views/user/UserListView.vue'),
  }
]

const router = new VueRouter({
  routes
})

export default router
