<template>
    <label class="swap swap-rotate" :class="{ 'swap-active': card.checked }" @click="swapCard">
        <div class="swap-off">
            <div class="card card-border shadow-xl bg-base-100 size-64">
                <div class="card-body">
                    <h2 class="card-title">{{ card.front }}</h2>
                </div>
            </div>
        </div>
        <div class="swap-on">
            <div class="card card-border shadow-xl bg-base-200 size-64">
                <div class="card-body">
                    <h2 class="card-title">
                        {{ card.back }}
                    </h2>
                </div>
            </div>
        </div>
    </label>
</template>

<script setup>
const props = defineProps({
    card: {
        type: Object,
        required: true,
        default: () => ({
            id: null,
            front: '',
            back: '',
            checked: false
        }),
        validator: (card) => {
            return card.front !== undefined && card.back !== undefined
        }
    }
});

const swapCard = () => {
    props.card.checked = !props.card.checked;
};

</script>