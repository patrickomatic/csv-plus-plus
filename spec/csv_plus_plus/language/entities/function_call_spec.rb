# frozen_string_literal: true

describe ::CSVPlusPlus::Language::Entities::FunctionCall do
  let(:function_call) { described_class.new('MINUS', [build(:number_one), build(:variable_foo)]) }
  subject { function_call }

  describe '#initialize' do
    it 'lowercases and converts the id to a symbol' do
      expect(subject.id).to(eq(:minus))
    end
  end

  describe '#function_call?' do
    it { is_expected.to(be_function_call) }
  end

  describe '#to_s' do
    subject { function_call.to_s }

    it { is_expected.to(eq('MINUS(1, $$foo)')) }

    context 'with no args' do
      let(:function_call) do
        build(
          :fn_call,
          name: :foo,
          arguments: [
            build(:cell_reference, ref: 'A'),
            build(:cell_reference, ref: 'B'),
            build(:cell_reference, ref: 'C')
          ]
        )
      end

      it { is_expected.to(eq('FOO(A, B, C)')) }
    end

    context 'with an infix function' do
      let(:function_call) do
        build(
          :fn_call,
          name: :*,
          arguments: [
            build(:cell_reference, ref: 'A'),
            build(:cell_reference, ref: 'B')
          ],
          infix: true
        )
      end

      it { is_expected.to(eq('(A * B)')) }
    end
  end

  describe '#==' do
    it { is_expected.to(eq(build(:fn_call, name: 'minus', arguments: [build(:number_one), build(:variable_foo)]))) }

    it { is_expected.not_to(eq(')')) }
    it { is_expected.not_to(eq(build(:fn_foo))) }
    it { is_expected.not_to(eq(build(:number_one))) }
    it { is_expected.not_to(eq(build(:variable_foo))) }
  end
end
