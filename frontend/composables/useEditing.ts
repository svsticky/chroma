const editing = ref(false)

export default function () {
  return {
    setEditing(value: boolean) {
      editing.value = value
    },
    isEditing() {
      return editing.value
    }
  }
}
