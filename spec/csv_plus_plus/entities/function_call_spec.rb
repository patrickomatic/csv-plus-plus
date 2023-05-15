# typed: false
# frozen_string_literal: true

describe ::CSVPlusPlus::Entities::FunctionCall do
  let(:entity) { described_class.new(:minus, [build(:number_one), build(:variable_foo)]) }

  subject { entity }

  describe '#==' do
    it { is_expected.to(eq(build(:fn_call, name: :minus, arguments: [build(:number_one), build(:variable_foo)]))) }

    it { is_expected.not_to(eq(build(:fn_foo))) }
    it { is_expected.not_to(eq(build(:number_one))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
  end

  describe '#evaluate' do
    let(:position) { build(:position) }

    subject { entity.evaluate(position) }

    it { is_expected.to(eq('MINUS(1, foo)')) }

    context 'with no args' do
      let(:entity) do
        build(
          :fn_call,
          name: :foo,
          arguments: [
            build(:reference, ref: 'A'),
            build(:reference, ref: 'B'),
            build(:reference, ref: 'C')
          ]
        )
      end

      it { is_expected.to(eq('FOO(A, B, C)')) }
    end

    context 'with an infix function' do
      let(:entity) do
        build(
          :fn_call,
          name: :*,
          arguments: [
            build(:reference, ref: 'A'),
            build(:reference, ref: 'B')
          ],
          infix: true
        )
      end

      it { is_expected.to(eq('(A * B)')) }
    end

    context 'with a nested function' do
      let(:entity) do
        build(
          :fn_call,
          name: :multiply,
          arguments: [
            build(:number, n: 5),
            build(:fn_call, name: :foo, arguments: [build(:number_one), build(:number_two)])
          ]
        )
      end

      it { is_expected.to(eq('MULTIPLY(5, FOO(1, 2))')) }
    end
  end
end
