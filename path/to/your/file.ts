const handleFocus = (index: number) => {
  ipParts.value[index] = ''
  const input = inputRefs.value[index]?.$el.querySelector('input')
  if (input instanceof HTMLInputElement) {
    input.value = '' // 清除输入框内容
    input.setAttribute('inputmode', 'numeric')
  }
}
