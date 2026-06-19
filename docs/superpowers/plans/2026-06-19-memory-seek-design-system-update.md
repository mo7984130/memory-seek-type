# 寻忆设计系统更新实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将现有"寻忆"网站的设计系统从棕色主题更新为青绿+淡紫主题，实现极简主义风格，支持深色/浅色模式。

**Architecture:** 基于现有 Vue 3 + Tailwind CSS v4 架构，更新 Tailwind 配置文件中的颜色系统，添加思源黑体字体，调整组件样式以实现极简主义设计。

**Tech Stack:** Vue 3, Tailwind CSS v4, Pinia, Vue Router, Lucide Vue Next

---

## Task 1: 更新 Tailwind 配置 - 配色系统

**Files:**
- Modify: `tailwind.config.js`

- [ ] **Step 1: 更新 primary 颜色为青绿色**

```javascript
// tailwind.config.js
colors: {
    primary: {
        DEFAULT: '#2DD4A8',  // 青绿主色
        light: '#5EEAD4',    // 浅青绿
        dark: '#14B8A6',     // 深青绿
        50: '#F0FDFA',
        100: '#CCFBF1',
        200: '#99F6E4',
        300: '#5EEAD4',
        400: '#2DD4A8',
        500: '#14B8A6',
        600: '#0D9488',
        700: '#0F766E',
        800: '#115E59',
        900: '#134E4A',
    },
    secondary: {
        DEFAULT: '#818CF8',  // 淡紫辅助色
        light: '#A5B4FC',
        dark: '#6366F1',
        50: '#EEF2FF',
        100: '#E0E7FF',
        200: '#C7D2FE',
        300: '#A5B4FC',
        400: '#818CF8',
        500: '#6366F1',
        600: '#4F46E5',
        700: '#4338CA',
        800: '#3730A3',
        900: '#312E81',
    },
    // ... 其他颜色保持不变
}
```

- [ ] **Step 2: 更新背景色为象牙白和深黑**

```javascript
backgroundColor: {
    'light': {
        primary: '#FFFFF0',    // 象牙白
        secondary: '#FAFAF5',  // 稍深的象牙白
        card: '#FFFFFF',       // 纯白卡片
        input: 'rgba(0, 0, 0, 0.03)',
    },
    'dark': {
        primary: '#121212',    // 深黑
        secondary: '#1E1E1E',  // 深灰
        card: '#252525',       // 卡片深灰
        input: 'rgba(255, 255, 255, 0.05)',
    },
},
```

- [ ] **Step 3: 更新文字颜色**

```javascript
textColor: {
    'light': {
        primary: '#1F2937',    // 深灰
        secondary: '#6B7280',  // 中灰
        tertiary: '#9CA3AF',   // 浅灰
        placeholder: '#D1D5DB',
    },
    'dark': {
        primary: '#F9FAFB',    // 浅灰
        secondary: '#9CA3AF',  // 中灰
        tertiary: '#6B7280',   // 深灰
        placeholder: '#4B5563',
    },
},
```

- [ ] **Step 4: 更新边框颜色**

```javascript
borderColor: {
    'light': '#E5E7EB',        // 浅灰边框
    'dark': '#374151',         // 深灰边框
    primary: {
        DEFAULT: 'rgba(45, 212, 168, 0.3)',  // 青绿边框
        dark: 'rgba(45, 212, 168, 0.4)',
    },
},
```

- [ ] **Step 5: 提交更改**

```bash
git add tailwind.config.js
git commit -m "feat: update tailwind config with new color system"
```

---

## Task 2: 添加思源黑体字体

**Files:**
- Modify: `index.html`
- Modify: `tailwind.config.js`

- [ ] **Step 1: 在 index.html 中添加 Google Fonts 链接**

```html
<!-- index.html -->
<head>
    <!-- 其他 meta 标签 -->
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Noto+Sans+SC:wght@300;400;500;600;700&display=swap" rel="stylesheet">
</head>
```

- [ ] **Step 2: 更新 tailwind.config.js 字体配置**

```javascript
// tailwind.config.js
fontFamily: {
    sans: [
        'Noto Sans SC',           // 思源黑体
        '-apple-system',
        'BlinkMacSystemFont',
        'Segoe UI',
        'Roboto',
        'Helvetica Neue',
        'Arial',
        'sans-serif',
    ],
    serif: ['Times New Roman', 'serif'],
},
```

- [ ] **Step 3: 提交更改**

```bash
git add index.html tailwind.config.js
git commit -m "feat: add Noto Sans SC font"
```

---

## Task 3: 更新全局样式 - 极简主义

**Files:**
- Modify: `src/assets/styles/global.scss` 或创建全局样式文件

- [ ] **Step 1: 创建或更新全局样式**

```scss
// src/assets/styles/global.scss
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
    // 浅色模式
    :root {
        --bg-primary: #FFFFF0;
        --bg-secondary: #FAFAF5;
        --bg-card: #FFFFFF;
        --text-primary: #1F2937;
        --text-secondary: #6B7280;
        --border-color: #E5E7EB;
        --primary-color: #2DD4A8;
        --secondary-color: #818CF8;
    }

    // 深色模式
    .dark {
        --bg-primary: #121212;
        --bg-secondary: #1E1E1E;
        --bg-card: #252525;
        --text-primary: #F9FAFB;
        --text-secondary: #9CA3AF;
        --border-color: #374151;
        --primary-color: #2DD4A8;
        --secondary-color: #818CF8;
    }

    body {
        @apply bg-light-primary dark:bg-dark-primary;
        @apply text-light-primary dark:text-dark-primary;
        font-family: 'Noto Sans SC', sans-serif;
    }
}

@layer components {
    // 卡片样式
    .card {
        @apply bg-light-card dark:bg-dark-card;
        @apply rounded-lg;
        @apply shadow-sm;
        @apply border border-light-border dark:border-dark-border;
    }

    // 按钮样式
    .btn-primary {
        @apply bg-primary text-white;
        @apply px-4 py-2 rounded-lg;
        @apply hover:bg-primary-dark;
        @apply transition-colors duration-200;
    }

    .btn-secondary {
        @apply bg-secondary text-white;
        @apply px-4 py-2 rounded-lg;
        @apply hover:bg-secondary-dark;
        @apply transition-colors duration-200;
    }
}
```

- [ ] **Step 2: 在 main.ts 中导入全局样式**

```typescript
// src/main.ts
import './assets/styles/global.scss'
```

- [ ] **Step 3: 提交更改**

```bash
git add src/assets/styles/global.scss src/main.ts
git commit -m "feat: add global styles with minimalist design"
```

---

## Task 4: 更新 MainLayout 导航栏

**Files:**
- Modify: `src/views/MainLayout.vue`

- [ ] **Step 1: 更新桌面端导航栏样式**

```vue
<!-- MainLayout.vue template 部分 -->
<template>
    <div class="min-h-screen bg-light-primary dark:bg-dark-primary">
        <!-- 顶部导航栏 -->
        <nav class="fixed top-0 left-0 right-0 z-50 bg-light-card dark:bg-dark-card border-b border-light-border dark:border-dark-border">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-16">
                    <!-- Logo -->
                    <div class="flex items-center">
                        <router-link to="/" class="flex items-center">
                            <span class="text-2xl font-bold text-primary">寻忆</span>
                        </router-link>
                    </div>

                    <!-- 桌面端导航链接 -->
                    <div class="hidden md:flex items-center space-x-8">
                        <router-link
                            v-for="item in navItems"
                            :key="item.path"
                            :to="item.path"
                            class="text-light-secondary dark:text-dark-secondary hover:text-primary dark:hover:text-primary transition-colors"
                            :class="{ 'text-primary font-medium': isActive(item.path) }"
                        >
                            {{ item.name }}
                        </router-link>
                    </div>

                    <!-- 右侧操作区 -->
                    <div class="flex items-center space-x-4">
                        <!-- 搜索按钮 -->
                        <button class="p-2 rounded-lg hover:bg-light-secondary dark:hover:bg-dark-secondary transition-colors">
                            <Search class="w-5 h-5" />
                        </button>

                        <!-- 主题切换 -->
                        <div class="hidden md:flex items-center space-x-2">
                            <button
                                v-for="mode in ['light', 'dark', 'system']"
                                :key="mode"
                                @click="setTheme(mode)"
                                class="p-2 rounded-lg transition-colors"
                                :class="currentMode === mode ? 'bg-primary text-white' : 'hover:bg-light-secondary dark:hover:bg-dark-secondary'"
                            >
                                <component :is="themeIcons[mode]" class="w-4 h-4" />
                            </button>
                        </div>

                        <!-- 用户头像 -->
                        <MenuComponent as="div" class="relative">
                            <MenuButton class="flex items-center">
                                <BaseAvatar :src="authStore.user?.avatar" :size="32" />
                            </MenuButton>
                            <Transition
                                enter-active-class="transition duration-100 ease-out"
                                enter-from-class="transform scale-95 opacity-0"
                                enter-to-class="transform scale-100 opacity-100"
                                leave-active-class="transition duration-75 ease-in"
                                leave-from-class="transform scale-100 opacity-100"
                                leave-to-class="transform scale-95 opacity-0"
                            >
                                <MenuItems class="absolute right-0 mt-2 w-48 rounded-lg shadow-lg bg-light-card dark:bg-dark-card border border-light-border dark:border-dark-border focus:outline-none">
                                    <!-- 菜单项 -->
                                </MenuItems>
                            </Transition>
                        </MenuComponent>
                    </div>
                </div>
            </div>
        </nav>

        <!-- 主内容区 -->
        <main class="pt-16">
            <RouterView />
        </main>

        <!-- 移动端底部导航 -->
        <nav class="md:hidden fixed bottom-0 left-0 right-0 bg-light-card dark:bg-dark-card border-t border-light-border dark:border-dark-border">
            <div class="flex justify-around py-2">
                <router-link
                    v-for="item in mobileNavItems"
                    :key="item.path"
                    :to="item.path"
                    class="flex flex-col items-center p-2 rounded-lg transition-colors"
                    :class="isActive(item.path) ? 'text-primary' : 'text-light-secondary dark:text-dark-secondary'"
                >
                    <component :is="item.icon" class="w-6 h-6" />
                    <span class="text-xs mt-1">{{ item.name }}</span>
                </router-link>
            </div>
        </nav>
    </div>
</template>
```

- [ ] **Step 2: 更新导航数据**

```typescript
// MainLayout.vue script 部分
const navItems = [
    { name: '首页', path: '/photos' },
    { name: '收藏夹', path: '/collections' },
    { name: '我喜欢', path: '/favorites' },
    { name: '个人主页', path: '/user' },
]

const mobileNavItems = [
    { name: '首页', path: '/photos', icon: Home },
    { name: '喜欢', path: '/favorites', icon: Heart },
    { name: '收藏', path: '/collections', icon: Star },
    { name: '我的', path: '/user', icon: User },
]

const themeIcons = {
    light: Sun,
    dark: Moon,
    system: Monitor,
}

const isActive = (path: string) => route.path === path || route.path.startsWith(path + '/')
```

- [ ] **Step 3: 提交更改**

```bash
git add src/views/MainLayout.vue
git commit -m "refactor: update navbar with minimalist design"
```

---

## Task 5: 更新照片卡片组件

**Files:**
- Modify: `src/components/PhotoCard.vue`

- [ ] **Step 1: 更新照片卡片样式**

```vue
<!-- PhotoCard.vue -->
<template>
    <div
        class="group relative rounded-lg overflow-hidden cursor-pointer transition-all duration-300 hover:shadow-lg"
        @click="$emit('click', photo)"
    >
        <!-- 图片 -->
        <img
            :src="photo.url"
            :alt="photo.description"
            class="w-full object-cover transition-transform duration-300 group-hover:scale-105"
            loading="lazy"
        />

        <!-- 桌面端悬停遮罩 -->
        <div class="hidden md:flex absolute inset-0 bg-black/0 group-hover:bg-black/40 transition-all duration-300 items-center justify-center opacity-0 group-hover:opacity-100">
            <div class="flex items-center space-x-4">
                <button
                    @click.stop="$emit('like', photo)"
                    class="p-2 rounded-full bg-white/20 hover:bg-white/40 transition-colors"
                >
                    <Heart class="w-5 h-5 text-white" />
                </button>
                <button
                    @click.stop="$emit('collect', photo)"
                    class="p-2 rounded-full bg-white/20 hover:bg-white/40 transition-colors"
                >
                    <Bookmark class="w-5 h-5 text-white" />
                </button>
                <button
                    @click.stop="$emit('share', photo)"
                    class="p-2 rounded-full bg-white/20 hover:bg-white/40 transition-colors"
                >
                    <Share2 class="w-5 h-5 text-white" />
                </button>
            </div>
        </div>

        <!-- 底部信息栏 -->
        <div class="absolute bottom-0 left-0 right-0 p-3 bg-gradient-to-t from-black/60 to-transparent">
            <div class="flex items-center justify-between text-white">
                <div class="flex items-center space-x-3">
                    <span class="flex items-center text-sm">
                        <Heart class="w-4 h-4 mr-1" />
                        {{ photo.likeCount }}
                    </span>
                    <span class="flex items-center text-sm">
                        <MessageCircle class="w-4 h-4 mr-1" />
                        {{ photo.commentCount }}
                    </span>
                </div>
            </div>
        </div>

        <!-- 移动端操作按钮（常驻） -->
        <div class="md:hidden absolute bottom-0 left-0 right-0 p-2 bg-gradient-to-t from-black/60 to-transparent">
            <div class="flex items-center justify-between">
                <div class="flex items-center space-x-2 text-white">
                    <span class="flex items-center text-xs">
                        <Heart class="w-3 h-3 mr-1" />
                        {{ photo.likeCount }}
                    </span>
                    <span class="flex items-center text-xs">
                        <MessageCircle class="w-3 h-3 mr-1" />
                        {{ photo.commentCount }}
                    </span>
                </div>
                <button
                    @click.stop="$emit('share', photo)"
                    class="p-1.5 rounded-full bg-white/20 hover:bg-white/40 transition-colors"
                >
                    <Share2 class="w-4 h-4 text-white" />
                </button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { Heart, Bookmark, Share2, MessageCircle } from 'lucide-vue-next'

defineProps({
    photo: {
        type: Object,
        required: true,
    },
})

defineEmits(['click', 'like', 'collect', 'share'])
</script>
```

- [ ] **Step 2: 提交更改**

```bash
git add src/components/PhotoCard.vue
git commit -m "refactor: update photo card with minimalist design"
```

---

## Task 6: 更新个人主页

**Files:**
- Modify: `src/views/user/UserProfile.vue`

- [ ] **Step 1: 更新个人主页布局**

```vue
<!-- UserProfile.vue -->
<template>
    <div class="max-w-4xl mx-auto px-4 py-8">
        <!-- 用户信息卡片 -->
        <div class="card p-6 mb-8">
            <div class="flex flex-col md:flex-row items-center md:items-start space-y-4 md:space-y-0 md:space-x-6">
                <!-- 头像 -->
                <BaseAvatar :src="user?.avatar" :size="96" />

                <!-- 用户信息 -->
                <div class="flex-1 text-center md:text-left">
                    <h1 class="text-2xl font-bold text-light-primary dark:text-dark-primary">
                        {{ user?.nickname || '用户' }}
                    </h1>
                    <p class="mt-2 text-light-secondary dark:text-dark-secondary">
                        {{ user?.bio || '这个人很懒，什么都没写~' }}
                    </p>

                    <!-- 统计数据 -->
                    <div class="flex justify-center md:justify-start space-x-8 mt-4">
                        <div class="text-center">
                            <div class="text-2xl font-bold text-primary">{{ stats.likeCount }}</div>
                            <div class="text-sm text-light-secondary dark:text-dark-secondary">获赞</div>
                        </div>
                        <div class="text-center">
                            <div class="text-2xl font-bold text-primary">{{ stats.collectCount }}</div>
                            <div class="text-sm text-light-secondary dark:text-dark-secondary">收藏</div>
                        </div>
                        <div class="text-center">
                            <div class="text-2xl font-bold text-primary">{{ stats.photoCount }}</div>
                            <div class="text-sm text-light-secondary dark:text-dark-secondary">照片</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>

        <!-- Tab 切换 -->
        <div class="flex space-x-1 mb-6 bg-light-secondary dark:bg-dark-secondary rounded-lg p-1">
            <button
                v-for="tab in tabs"
                :key="tab.key"
                @click="activeTab = tab.key"
                class="flex-1 py-2 px-4 rounded-md transition-all duration-200"
                :class="activeTab === tab.key ? 'bg-primary text-white shadow-sm' : 'text-light-secondary dark:text-dark-secondary hover:text-primary'"
            >
                {{ tab.name }}
            </button>
        </div>

        <!-- 内容区 -->
        <div class="mt-6">
            <component :is="tabComponents[activeTab]" />
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Heart, Star, Image } from 'lucide-vue-next'
import useAuthStore from '@/util/memory-seek-api/stores/auth-store.ts'
import BaseAvatar from '@/components/base/BaseAvatar.vue'
import MyLikes from './components/MyLikes.vue'
import MyCollections from './components/MyCollections.vue'
import MyPhotos from './components/MyPhotos.vue'

const authStore = useAuthStore()
const user = computed(() => authStore.user)

const activeTab = ref('likes')

const tabs = [
    { key: 'likes', name: '我喜欢', icon: Heart },
    { key: 'collections', name: '收藏夹', icon: Star },
    { key: 'photos', name: '我的照片', icon: Image },
]

const tabComponents = {
    likes: MyLikes,
    collections: MyCollections,
    photos: MyPhotos,
}

// 统计数据（需要从 API 获取）
const stats = ref({
    likeCount: 0,
    collectCount: 0,
    photoCount: 0,
})
</script>
```

- [ ] **Step 2: 创建子组件**

```vue
<!-- src/views/user/components/MyLikes.vue -->
<template>
    <div>
        <!-- 瀑布流展示我喜欢的照片 -->
        <PhotoWaterfall :photos="likedPhotos" @like="handleUnlike" />
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import PhotoWaterfall from '@/components/PhotoWaterfall.vue'

const likedPhotos = ref([])

// 获取我喜欢的照片
const fetchLikedPhotos = async () => {
    // API 调用
}

const handleUnlike = async (photo) => {
    // 取消点赞逻辑
}

onMounted(() => {
    fetchLikedPhotos()
})
</script>
```

- [ ] **Step 3: 提交更改**

```bash
git add src/views/user/UserProfile.vue src/views/user/components/
git commit -m "refactor: update user profile with new design"
```

---

## Task 7: 更新照片详情页

**Files:**
- Modify: `src/components/CustomPhotoViewer.vue`

- [ ] **Step 1: 更新照片详情页样式**

```vue
<!-- CustomPhotoViewer.vue 关键部分 -->
<template>
    <div class="fixed inset-0 z-50 bg-black/90 flex items-center justify-center">
        <!-- 返回按钮 -->
        <button
            @click="$emit('close')"
            class="absolute top-4 left-4 p-2 rounded-full bg-white/10 hover:bg-white/20 transition-colors"
        >
            <ArrowLeft class="w-6 h-6 text-white" />
        </button>

        <!-- 图片容器 -->
        <div class="max-w-4xl max-h-[80vh] mx-auto">
            <img
                :src="photo.url"
                :alt="photo.description"
                class="max-w-full max-h-[80vh] object-contain"
            />
        </div>

        <!-- 操作栏 -->
        <div class="absolute bottom-8 left-1/2 transform -translate-x-1/2 flex items-center space-x-6">
            <button
                @click="$emit('like', photo)"
                class="flex items-center space-x-2 px-4 py-2 rounded-full bg-white/10 hover:bg-white/20 transition-colors"
            >
                <Heart class="w-5 h-5 text-white" />
                <span class="text-white">{{ photo.likeCount }}</span>
            </button>
            <button
                @click="$emit('collect', photo)"
                class="flex items-center space-x-2 px-4 py-2 rounded-full bg-white/10 hover:bg-white/20 transition-colors"
            >
                <Bookmark class="w-5 h-5 text-white" />
                <span class="text-white">收藏</span>
            </button>
            <button
                @click="$emit('share', photo)"
                class="flex items-center space-x-2 px-4 py-2 rounded-full bg-white/10 hover:bg-white/20 transition-colors"
            >
                <Share2 class="w-5 h-5 text-white" />
                <span class="text-white">分享</span>
            </button>
        </div>

        <!-- 评论区（默认折叠） -->
        <div class="absolute right-4 bottom-4 top-4 w-80 bg-light-card dark:bg-dark-card rounded-lg overflow-hidden flex flex-col">
            <!-- 评论头部 -->
            <button
                @click="showComments = !showComments"
                class="flex items-center justify-between p-4 border-b border-light-border dark:border-dark-border"
            >
                <span class="font-medium text-light-primary dark:text-dark-primary">
                    评论 ({{ photo.commentCount }})
                </span>
                <ChevronDown
                    class="w-5 h-5 text-light-secondary dark:text-dark-secondary transition-transform"
                    :class="{ 'rotate-180': showComments }"
                />
            </button>

            <!-- 评论列表 -->
            <div v-show="showComments" class="flex-1 overflow-y-auto p-4">
                <div v-for="comment in comments" :key="comment.id" class="mb-4">
                    <div class="flex items-start space-x-3">
                        <BaseAvatar :src="comment.user.avatar" :size="32" />
                        <div>
                            <div class="font-medium text-light-primary dark:text-dark-primary">
                                {{ comment.user.nickname }}
                            </div>
                            <div class="text-light-secondary dark:text-dark-secondary">
                                {{ comment.content }}
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <!-- 评论输入框 -->
            <div v-show="showComments" class="p-4 border-t border-light-border dark:border-dark-border">
                <div class="flex space-x-2">
                    <input
                        v-model="commentText"
                        placeholder="写评论..."
                        class="flex-1 px-3 py-2 rounded-lg bg-light-secondary dark:bg-dark-secondary border border-light-border dark:border-dark-border focus:outline-none focus:border-primary"
                    />
                    <button
                        @click="submitComment"
                        class="px-4 py-2 bg-primary text-white rounded-lg hover:bg-primary-dark transition-colors"
                    >
                        发送
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { ArrowLeft, Heart, Bookmark, Share2, ChevronDown } from 'lucide-vue-next'
import BaseAvatar from '@/components/base/BaseAvatar.vue'

const props = defineProps({
    photo: {
        type: Object,
        required: true,
    },
})

const emit = defineEmits(['close', 'like', 'collect', 'share', 'comment'])

const showComments = ref(false)
const commentText = ref('')
const comments = ref([])

const submitComment = () => {
    if (commentText.value.trim()) {
        emit('comment', commentText.value)
        commentText.value = ''
    }
}
</script>
```

- [ ] **Step 2: 提交更改**

```bash
git add src/components/CustomPhotoViewer.vue
git commit -m "refactor: update photo viewer with collapsible comments"
```

---

## Task 8: 更新收藏夹和我喜欢页面

**Files:**
- Modify: `src/views/CollectionView.vue`
- Modify: `src/views/FavoritesView.vue`

- [ ] **Step 1: 更新 CollectionView**

```vue
<!-- CollectionView.vue -->
<template>
    <div class="max-w-7xl mx-auto px-4 py-8">
        <h1 class="text-2xl font-bold text-light-primary dark:text-dark-primary mb-6">收藏夹</h1>

        <!-- 收藏夹网格 -->
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            <div
                v-for="collection in collections"
                :key="collection.id"
                class="card overflow-hidden cursor-pointer hover:shadow-lg transition-shadow"
                @click="goToCollection(collection.id)"
            >
                <!-- 封面图 -->
                <div class="aspect-video bg-light-secondary dark:bg-dark-secondary">
                    <img
                        v-if="collection.cover"
                        :src="collection.cover"
                        :alt="collection.name"
                        class="w-full h-full object-cover"
                    />
                </div>

                <!-- 信息 -->
                <div class="p-4">
                    <h3 class="font-medium text-light-primary dark:text-dark-primary">
                        {{ collection.name }}
                    </h3>
                    <p class="text-sm text-light-secondary dark:text-dark-secondary mt-1">
                        {{ collection.photoCount }} 张照片
                    </p>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const collections = ref([])

const fetchCollections = async () => {
    // API 调用
}

const goToCollection = (id: string) => {
    router.push(`/collection/${id}`)
}

onMounted(() => {
    fetchCollections()
})
</script>
```

- [ ] **Step 2: 更新 FavoritesView**

```vue
<!-- FavoritesView.vue -->
<template>
    <div class="max-w-7xl mx-auto px-4 py-8">
        <h1 class="text-2xl font-bold text-light-primary dark:text-dark-primary mb-6">我喜欢</h1>

        <!-- 瀑布流展示 -->
        <PhotoWaterfall
            :photos="favoritePhotos"
            @like="handleUnlike"
            @click="goToDetail"
        />
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import PhotoWaterfall from '@/components/PhotoWaterfall.vue'

const router = useRouter()
const favoritePhotos = ref([])

const fetchFavorites = async () => {
    // API 调用
}

const handleUnlike = async (photo) => {
    // 取消点赞逻辑
}

const goToDetail = (photo) => {
    // 打开详情页
}

onMounted(() => {
    fetchFavorites()
})
</script>
```

- [ ] **Step 3: 提交更改**

```bash
git add src/views/CollectionView.vue src/views/FavoritesView.vue
git commit -m "refactor: update collections and favorites views"
```

---

## Task 9: 测试和验证

- [ ] **Step 1: 启动开发服务器**

```bash
cd /home/dr/dev/source/memory-seek/memory-seek-front
pnpm dev
```

- [ ] **Step 2: 检查浅色模式**

- 访问 http://localhost:5173
- 验证象牙白背景 (#FFFFF0)
- 验证青绿色主色 (#2DD4A8)
- 验证淡紫色辅助色 (#818CF8)
- 验证思源黑体字体
- 测试所有页面布局

- [ ] **Step 3: 检查深色模式**

- 切换到深色模式
- 验证深黑背景 (#121212)
- 验证所有文字颜色正确
- 验证边框颜色正确

- [ ] **Step 4: 检查响应式设计**

- 测试桌面端 (≥1024px)
- 测试平板端 (768px-1023px)
- 测试移动端 (<768px)
- 验证底部导航栏显示

- [ ] **Step 5: 测试交互功能**

- 测试照片卡片悬停效果
- 测试评论区展开/收起
- 测试主题切换
- 测试页面导航

- [ ] **Step 6: 提交最终更改**

```bash
git add .
git commit -m "feat: complete design system update for memory-seek"
```

---

## 检查清单

- [ ] 配色系统已更新
- [ ] 思源黑体已添加
- [ ] 极简主义样式已应用
- [ ] 深色/浅色模式正常工作
- [ ] 响应式设计正常
- [ ] 所有页面已更新
- [ ] 交互功能正常
- [ ] 代码已提交
